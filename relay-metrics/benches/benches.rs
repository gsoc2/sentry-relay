use std::collections::BTreeMap;
use std::fmt;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use relay_base_schema::project::ProjectKey;
use relay_common::time::UnixTimestamp;
use relay_metrics::{AggregatorConfig, AggregatorService, DistributionValue, Metric, MetricValue};

/// Struct representing a testcase for which insert + flush are timed.
struct MetricInput {
    num_metrics: usize,
    num_metric_names: usize,
    num_project_keys: usize,
    metric: Metric,
}

impl MetricInput {
    // Separate from actual metric insertion as we do not want to time this function, its logic and
    // especially not creation and destruction of a large vector.
    //
    // In theory we could also create all vectors upfront instead of having a MetricInput struct,
    // but that would take a lot of memory. This way we can at least free some RAM between
    // benchmarks.
    fn get_metrics(&self) -> Vec<(ProjectKey, Metric)> {
        let mut rv = Vec::new();

        for i in 0..self.num_metrics {
            let key_id = i % self.num_project_keys;
            let metric_name = format!("c:transactions/foo{}", i % self.num_metric_names);
            let mut metric = self.metric.clone();
            metric.name = metric_name;
            let key = ProjectKey::parse(&format!("{key_id:0width$x}", width = 32)).unwrap();
            rv.push((key, metric));
        }

        rv
    }
}

impl fmt::Display for MetricInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:?} metrics with {} names, {} keys",
            self.num_metrics,
            self.metric.value.ty(),
            self.num_metric_names,
            self.num_project_keys
        )
    }
}

fn bench_insert_and_flush(c: &mut Criterion) {
    let config = AggregatorConfig {
        bucket_interval: 1000,
        initial_delay: 0,
        debounce_delay: 0,
        ..Default::default()
    };

    let counter = Metric {
        name: "c:transactions/foo@none".to_owned(),
        value: MetricValue::Counter(42.),
        timestamp: UnixTimestamp::now(),
        tags: BTreeMap::new(),
    };

    let inputs = [
        MetricInput {
            num_metrics: 1,
            num_metric_names: 1,
            metric: counter.clone(),
            num_project_keys: 1,
        },
        // scaling num_metrics
        MetricInput {
            num_metrics: 100,
            num_metric_names: 1,
            metric: counter.clone(),
            num_project_keys: 1,
        },
        MetricInput {
            num_metrics: 1000,
            num_metric_names: 1,
            metric: counter.clone(),
            num_project_keys: 1,
        },
        // scaling num_metric_names
        MetricInput {
            num_metrics: 100,
            num_metric_names: 100,
            metric: counter.clone(),
            num_project_keys: 1,
        },
        MetricInput {
            num_metrics: 1000,
            num_metric_names: 1000,
            metric: counter.clone(),
            num_project_keys: 1,
        },
        // scaling num_project_keys
        MetricInput {
            num_metrics: 100,
            num_metric_names: 1,
            metric: counter.clone(),
            num_project_keys: 100,
        },
        MetricInput {
            num_metrics: 1000,
            num_metric_names: 1,
            metric: counter,
            num_project_keys: 1000,
        },
    ];

    for input in &inputs {
        c.bench_with_input(
            BenchmarkId::new("bench_insert_metrics", input),
            &input,
            |b, &input| {
                b.iter_batched(
                    || {
                        (
                            AggregatorService::new(config.clone(), None),
                            input.get_metrics(),
                        )
                    },
                    |(mut aggregator, metrics)| {
                        for (project_key, metric) in metrics {
                            aggregator.insert(project_key, metric).unwrap();
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        c.bench_with_input(
            BenchmarkId::new("bench_flush_metrics", input),
            &input,
            |b, &input| {
                b.iter_batched(
                    || {
                        let mut aggregator = AggregatorService::new(config.clone(), None);
                        for (project_key, metric) in input.get_metrics() {
                            aggregator.insert(project_key, metric).unwrap();
                        }
                        aggregator
                    },
                    |mut aggregator| {
                        // XXX: Ideally we'd want to test the entire try_flush here, but spawning
                        // a service is too much work here.
                        aggregator.pop_flush_buckets();
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
}

fn bench_distribution(c: &mut Criterion) {
    let mut group = c.benchmark_group("DistributionValue");

    for size in [1, 10, 100, 1000, 10_000, 100_000, 1_000_000] {
        let values = std::iter::from_fn(|| Some(rand::random()))
            .take(size as usize)
            .collect::<Vec<f64>>();

        group.throughput(criterion::Throughput::Elements(size));
        group.bench_with_input(BenchmarkId::from_parameter(size), &values, |b, values| {
            b.iter(|| DistributionValue::from_iter(black_box(values.iter().copied())))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_insert_and_flush, bench_distribution);
criterion_main!(benches);
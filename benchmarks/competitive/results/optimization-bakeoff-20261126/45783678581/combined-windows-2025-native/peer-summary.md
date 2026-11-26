| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.56x | 1.53x | 1.43x | 2.15x | 1.87x | 30/30 |
| materialize | 0.99x | 2.03x | 1.33x | 1.15x | 2.28x | 1.44x | 30/30 |
| perft_bulk_d4 | 0.96x | 3.98x | 1.48x | 1.25x | 2.29x | 2.14x | 30/30 |
| perft_full_d3 | 0.90x | 1.09x | 1.21x | 0.99x | 1.63x | 1.72x | 23/30 |
| fen_parse | 0.99x | 1.38x | 1.61x | 1.59x | 2.20x | 1.69x | 30/30 |
| fen_write_native_ep | 1.00x | 3.21x | 6.23x | 1.30x | 27.01x | 1.73x | 30/30 |
| snapshot | 0.83x | 1.82x | 1.08x | 3.03x | 1.07x | 1.00x | 27/30 |
| san_parse_all | 0.97x | 1.98x | N/A | 4.62x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 10.66x | N/A | 3.68x | N/A | N/A | 12/12 |
| apply_children_full | 0.82x | 1.53x | 1.12x | 6.49x | 1.11x | 1.56x | 28/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.26x | 90.67x | 182.17x | 240.73x | 202.93x | 5/5 |
| chess960_materialize | 1.00x | 1.44x | 1.17x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.74x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 36.81x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 24.82x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.39x | 1.86x | 1.27x | 2.56x | 2.43x | 10/10 |
| corpus_materialize | 0.94x | 1.75x | 1.34x | 1.15x | 2.26x | 1.39x | 10/10 |
| corpus_uci_sorted | 0.95x | 1.53x | 1.87x | 1.62x | 1.83x | 1.29x | 10/10 |
| replay_trusted_native | 1.03x | 0.67x | 1.25x | 1.43x | 2.47x | 3.26x | 8/10 |
| replay_checked_native | 1.22x | 2.32x | 1.10x | 8.44x | 8.15x | 11.01x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.

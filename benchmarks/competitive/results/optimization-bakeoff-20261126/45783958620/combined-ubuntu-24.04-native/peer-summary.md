| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.00x | 1.71x | 1.46x | 2.25x | 1.76x | 30/30 |
| materialize | 1.00x | 2.10x | 1.62x | 1.64x | 2.51x | 1.87x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.86x | 1.58x | 1.20x | 2.38x | 1.90x | 30/30 |
| perft_full_d3 | 0.98x | 1.46x | 1.67x | 1.62x | 2.22x | 1.78x | 30/30 |
| fen_parse | 1.13x | 1.53x | 1.63x | 1.24x | 1.64x | 1.16x | 30/30 |
| fen_write_native_ep | 1.00x | 4.62x | 10.18x | 2.07x | 28.27x | 2.51x | 30/30 |
| snapshot | 0.91x | 1.73x | 1.08x | 3.06x | 1.09x | 1.16x | 30/30 |
| san_parse_all | 1.06x | 2.00x | N/A | 4.68x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 9.26x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.66x | 1.05x | 3.04x | 1.02x | 1.28x | 27/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.08x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.16x | 72.28x | 132.86x | 158.13x | 126.83x | 5/5 |
| chess960_materialize | 0.97x | 1.62x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.25x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 40.27x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 20.14x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.41x | 2.45x | 1.33x | 2.95x | 2.70x | 10/10 |
| corpus_materialize | 1.01x | 1.42x | 1.33x | 1.10x | 1.73x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.24x | 2.90x | 2.21x | 2.84x | 1.55x | 10/10 |
| replay_trusted_native | 0.93x | 0.65x | 1.24x | 1.21x | 1.68x | 1.96x | 7/10 |
| replay_checked_native | 0.99x | 1.97x | 1.11x | 8.47x | 8.05x | 9.61x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.

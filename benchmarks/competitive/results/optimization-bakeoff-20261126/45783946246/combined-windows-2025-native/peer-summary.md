| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.12x | 1.73x | 1.50x | 2.05x | 1.80x | 30/30 |
| materialize | 1.00x | 1.98x | 1.38x | 1.54x | 2.34x | 1.59x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.99x | 1.61x | 1.23x | 2.21x | 1.92x | 30/30 |
| perft_full_d3 | 0.99x | 1.61x | 1.78x | 1.64x | 2.27x | 1.89x | 30/30 |
| fen_parse | 0.96x | 1.26x | 1.56x | 1.39x | 1.76x | 1.47x | 30/30 |
| fen_write_native_ep | 0.99x | 4.19x | 7.84x | 1.72x | 30.63x | 2.19x | 30/30 |
| snapshot | 1.04x | 1.92x | 1.23x | 4.16x | 1.13x | 1.13x | 30/30 |
| san_parse_all | 1.04x | 1.78x | N/A | 5.44x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.53x | N/A | 4.08x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.68x | 1.14x | 8.48x | 1.12x | 1.36x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.07x | N/A | N/A | 4/6 |
| startpos | 1.00x | 0.98x | 64.86x | 137.04x | 172.84x | 145.98x | 4/5 |
| chess960_materialize | 1.01x | 1.56x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.51x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 41.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.04x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 24.98x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 6.63x | 2.67x | 1.32x | 2.93x | 2.86x | 10/10 |
| corpus_materialize | 1.04x | 1.47x | 1.28x | 1.13x | 1.84x | 1.24x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.81x | 2.19x | 1.84x | 2.14x | 1.53x | 10/10 |
| replay_trusted_native | 1.02x | 0.70x | 1.29x | 1.35x | 1.80x | 2.06x | 8/10 |
| replay_checked_native | 1.02x | 2.12x | 1.08x | 8.42x | 8.25x | 9.56x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.

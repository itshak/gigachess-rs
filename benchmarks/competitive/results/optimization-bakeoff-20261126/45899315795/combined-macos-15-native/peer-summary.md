| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.86x | 4.67x | 1.50x | 1.62x | 2.15x | 1.85x | 30/30 |
| materialize | 1.10x | 2.02x | 1.57x | 1.06x | 2.21x | 1.32x | 27/30 |
| perft_bulk_d4 | 1.00x | 4.04x | 1.48x | 1.24x | 1.86x | 1.85x | 29/30 |
| perft_full_d3 | 0.88x | 1.36x | 1.47x | 1.15x | 1.65x | 1.76x | 29/30 |
| fen_parse | 0.99x | 1.21x | 1.37x | 1.27x | 1.63x | 1.44x | 29/30 |
| fen_write_native_ep | 0.93x | 4.03x | 8.51x | 1.68x | 28.91x | 2.15x | 30/30 |
| snapshot | 0.98x | 1.61x | 1.02x | 2.15x | 1.05x | 0.91x | 23/30 |
| san_parse_all | 1.12x | 1.88x | N/A | 3.97x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 12.65x | N/A | 2.84x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 1.23x | 1.33x | 6.33x | 1.23x | 1.42x | 29/30 |
| make_unmake_full | 0.90x | N/A | N/A | 1.19x | N/A | N/A | 5/6 |
| startpos | 1.01x | 1.20x | 68.00x | 128.62x | 152.90x | 138.98x | 5/5 |
| chess960_materialize | 1.04x | 1.73x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.23x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 38.94x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.90x | 0.93x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 20.66x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.77x | 1.93x | 1.10x | 2.26x | 2.33x | 9/10 |
| corpus_materialize | 0.98x | 1.50x | 1.38x | 1.18x | 1.97x | 1.27x | 10/10 |
| corpus_uci_sorted | 1.06x | 1.91x | 2.65x | 2.13x | 2.63x | 1.48x | 10/10 |
| replay_trusted_native | 0.96x | 0.68x | 1.62x | 1.51x | 1.78x | 1.67x | 8/10 |
| replay_checked_native | 0.94x | 2.42x | 1.15x | 8.31x | 7.82x | 9.52x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

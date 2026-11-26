| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.50x | 1.88x | 1.64x | 2.51x | 1.93x | 30/30 |
| materialize | 1.06x | 2.28x | 1.70x | 1.77x | 2.70x | 1.92x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.82x | 1.51x | 1.19x | 2.34x | 1.87x | 29/30 |
| perft_full_d3 | 1.21x | 1.66x | 1.89x | 1.82x | 2.38x | 2.73x | 30/30 |
| fen_parse | 1.01x | 1.36x | 1.33x | 1.06x | 1.45x | 1.14x | 30/30 |
| fen_write_native_ep | 1.00x | 4.88x | 10.07x | 2.12x | 28.84x | 2.68x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 4.33x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 1.01x | 1.89x | N/A | 4.34x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 8.96x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.12x | 1.34x | 4.02x | 1.34x | 1.68x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.11x | 87.86x | 152.92x | 195.05x | 161.51x | 5/5 |
| chess960_materialize | 1.01x | 1.81x | 1.72x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.50x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 39.64x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.86x | 19.58x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.45x | 2.03x | 1.12x | 2.54x | 2.26x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.34x | 1.09x | 1.76x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.27x | 3.06x | 2.24x | 3.00x | 1.55x | 10/10 |
| replay_trusted_native | 0.94x | 0.86x | 1.53x | 1.49x | 2.10x | 2.42x | 9/10 |
| replay_checked_native | 0.96x | 2.14x | 1.11x | 8.86x | 8.83x | 10.01x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

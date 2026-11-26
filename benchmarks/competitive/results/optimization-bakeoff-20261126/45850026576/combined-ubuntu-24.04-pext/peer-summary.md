| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.48x | 1.87x | 1.63x | 2.51x | 1.92x | 30/30 |
| materialize | 1.00x | 2.26x | 1.74x | 1.76x | 2.89x | 1.94x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.82x | 1.52x | 1.21x | 2.36x | 1.85x | 29/30 |
| perft_full_d3 | 1.01x | 1.41x | 1.55x | 1.52x | 1.99x | 1.65x | 30/30 |
| fen_parse | 1.01x | 1.26x | 1.33x | 1.15x | 1.48x | 1.15x | 30/30 |
| fen_write_native_ep | 1.00x | 5.03x | 10.64x | 1.94x | 30.00x | 2.17x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.76x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 8.98x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.12x | 1.33x | 4.05x | 1.35x | 1.67x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.11x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.10x | 87.98x | 162.57x | 196.96x | 160.23x | 5/5 |
| chess960_materialize | 1.02x | 1.76x | 1.71x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.52x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 39.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.02x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.97x | 22.77x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 5.44x | 2.02x | 1.14x | 2.47x | 2.28x | 10/10 |
| corpus_materialize | 1.00x | 1.42x | 1.31x | 1.08x | 1.75x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.31x | 3.06x | 2.27x | 3.03x | 1.59x | 10/10 |
| replay_trusted_native | 0.98x | 0.88x | 1.60x | 1.61x | 2.25x | 2.55x | 9/10 |
| replay_checked_native | 0.98x | 1.95x | 1.03x | 8.16x | 8.00x | 9.26x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

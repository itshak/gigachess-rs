| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.92x | 4.54x | 1.45x | 1.44x | 2.00x | 1.82x | 28/30 |
| materialize | 1.10x | 1.99x | 1.47x | 1.11x | 2.26x | 1.37x | 26/30 |
| perft_bulk_d4 | 0.81x | 3.90x | 1.45x | 1.18x | 1.80x | 1.64x | 29/30 |
| perft_full_d3 | 0.91x | 1.59x | 1.79x | 1.34x | 1.96x | 2.12x | 30/30 |
| fen_parse | 0.88x | 1.30x | 1.29x | 1.30x | 1.61x | 1.50x | 30/30 |
| fen_write_native_ep | 0.85x | 4.11x | 9.24x | 1.64x | 27.91x | 2.20x | 30/30 |
| snapshot | 1.01x | 1.66x | 1.26x | 2.68x | 1.28x | 0.89x | 24/30 |
| san_parse_all | 0.93x | 2.10x | N/A | 4.97x | N/A | N/A | 12/12 |
| san_render_native | 0.87x | 12.28x | N/A | 2.72x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 1.03x | 1.06x | 5.41x | 0.98x | 1.19x | 23/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.18x | N/A | N/A | 5/6 |
| startpos | 0.56x | 1.02x | 48.48x | 111.04x | 114.08x | 101.48x | 5/5 |
| chess960_materialize | 0.85x | 1.37x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.79x | 2.68x | 1.15x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.24x | 46.00x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 1.27x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.06x | 19.76x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 6.39x | 2.35x | 1.59x | 2.50x | 2.56x | 10/10 |
| corpus_materialize | 1.21x | 1.58x | 1.38x | 1.12x | 1.95x | 1.28x | 10/10 |
| corpus_uci_sorted | 1.12x | 1.78x | 2.54x | 1.93x | 2.50x | 1.49x | 10/10 |
| replay_trusted_native | 0.90x | 0.57x | 1.38x | 1.25x | 1.56x | 1.74x | 8/10 |
| replay_checked_native | 1.08x | 2.13x | 1.05x | 8.06x | 6.95x | 6.99x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.

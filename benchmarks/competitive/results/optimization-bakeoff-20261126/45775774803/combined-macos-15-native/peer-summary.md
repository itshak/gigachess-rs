| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.07x | 4.67x | 1.45x | 1.52x | 1.99x | 1.70x | 30/30 |
| materialize | 0.98x | 1.90x | 1.38x | 1.00x | 2.67x | 1.28x | 27/30 |
| perft_bulk_d4 | 0.93x | 4.15x | 1.49x | 1.24x | 1.91x | 1.82x | 29/30 |
| perft_full_d3 | 1.06x | 0.91x | 1.04x | 0.84x | 1.11x | 1.22x | 20/30 |
| fen_parse | 0.98x | 1.17x | 1.33x | 1.22x | 1.50x | 1.36x | 28/30 |
| fen_write_native_ep | 0.96x | 2.71x | 6.49x | 1.20x | 19.47x | 1.57x | 29/30 |
| snapshot | 0.87x | 1.35x | 0.82x | 1.85x | 0.84x | 0.77x | 16/30 |
| san_parse_all | 1.03x | 1.68x | N/A | 3.53x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 12.24x | N/A | 2.64x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 0.80x | 0.92x | 4.24x | 0.86x | 0.96x | 9/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.01x | N/A | N/A | 2/6 |
| startpos | 1.03x | 1.25x | 70.34x | 138.56x | 163.84x | 142.93x | 5/5 |
| chess960_materialize | 1.22x | 1.43x | 1.25x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.12x | 2.80x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.92x | 37.46x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.92x | 19.20x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 6.77x | 2.08x | 1.36x | 2.58x | 2.72x | 10/10 |
| corpus_materialize | 1.13x | 1.48x | 1.41x | 1.12x | 1.82x | 1.25x | 10/10 |
| corpus_uci_sorted | 0.95x | 1.17x | 1.72x | 1.34x | 1.68x | 0.99x | 9/10 |
| replay_trusted_native | 0.83x | 0.42x | 1.16x | 1.10x | 1.04x | 1.20x | 7/10 |
| replay_checked_native | 0.96x | 2.19x | 1.03x | 7.65x | 6.21x | 9.49x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.

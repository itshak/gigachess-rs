| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 5.26x | 1.75x | 1.88x | 2.44x | 2.07x | 30/30 |
| materialize | 0.94x | 2.30x | 1.67x | 1.14x | 2.39x | 1.45x | 30/30 |
| perft_bulk_d4 | 0.94x | 4.22x | 1.48x | 1.30x | 2.07x | 1.77x | 30/30 |
| perft_full_d3 | 0.98x | 1.56x | 1.68x | 1.36x | 1.95x | 1.90x | 30/30 |
| fen_parse | 0.98x | 1.13x | 1.46x | 1.20x | 1.58x | 1.40x | 30/30 |
| fen_write_native_ep | 1.06x | 4.19x | 8.95x | 1.74x | 27.36x | 2.34x | 30/30 |
| snapshot | 0.98x | 1.76x | 1.09x | 2.54x | 1.09x | 1.05x | 26/30 |
| san_parse_all | 0.98x | 1.78x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 12.78x | N/A | 2.83x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.13x | 1.24x | 5.92x | 1.20x | 1.37x | 26/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.44x | 1.10x | 69.01x | 142.11x | 181.89x | 171.11x | 5/5 |
| chess960_materialize | 0.93x | 1.64x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.87x | 3.26x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 38.88x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.10x | 1.11x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.92x | 21.84x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.80x | 1.84x | 1.18x | 2.21x | 2.54x | 10/10 |
| corpus_materialize | 1.01x | 1.42x | 1.39x | 1.16x | 2.03x | 1.34x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.86x | 2.68x | 2.08x | 2.67x | 1.48x | 10/10 |
| replay_trusted_native | 0.91x | 0.64x | 1.65x | 1.58x | 1.94x | 1.91x | 8/10 |
| replay_checked_native | 1.05x | 2.82x | 1.30x | 10.03x | 9.33x | 10.37x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

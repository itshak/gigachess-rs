| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 4.33x | 1.60x | 1.47x | 2.09x | 1.88x | 30/30 |
| materialize | 1.14x | 1.89x | 1.48x | 1.12x | 2.36x | 1.35x | 28/30 |
| perft_bulk_d4 | 1.08x | 3.42x | 1.34x | 1.14x | 1.66x | 1.71x | 27/30 |
| perft_full_d3 | 1.06x | 1.51x | 1.64x | 1.26x | 1.80x | 1.87x | 29/30 |
| fen_parse | 1.00x | 1.19x | 1.38x | 1.37x | 1.65x | 1.32x | 30/30 |
| fen_write_native_ep | 1.04x | 4.39x | 8.58x | 1.93x | 28.02x | 2.32x | 30/30 |
| snapshot | 1.02x | 1.73x | 1.07x | 2.32x | 1.08x | 0.86x | 22/30 |
| san_parse_all | 0.93x | 1.82x | N/A | 3.64x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 11.83x | N/A | 2.77x | N/A | N/A | 12/12 |
| apply_children_full | 0.85x | 1.11x | 1.12x | 5.32x | 1.05x | 1.26x | 27/30 |
| make_unmake_full | 0.87x | N/A | N/A | 1.16x | N/A | N/A | 5/6 |
| startpos | 1.34x | 1.12x | 83.15x | 174.83x | 209.10x | 158.37x | 5/5 |
| chess960_materialize | 0.89x | 1.50x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.83x | 2.45x | 1.21x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.84x | 35.02x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.83x | 1.11x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.83x | 19.46x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.81x | 4.89x | 1.78x | 0.92x | 1.87x | 1.97x | 9/10 |
| corpus_materialize | 0.98x | 1.61x | 1.41x | 1.27x | 2.00x | 1.38x | 10/10 |
| corpus_uci_sorted | 0.74x | 1.50x | 2.08x | 1.66x | 2.04x | 1.17x | 10/10 |
| replay_trusted_native | 0.63x | 0.49x | 1.23x | 1.37x | 1.41x | 1.58x | 7/10 |
| replay_checked_native | 0.83x | 2.08x | 1.13x | 9.08x | 8.07x | 9.87x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

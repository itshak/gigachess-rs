| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 2.41x | 1.39x | 1.20x | 1.61x | 1.15x | 28/30 |
| materialize | 1.04x | 2.05x | 1.60x | 1.72x | 2.40x | 1.80x | 30/30 |
| perft_bulk_d4 | 1.07x | 2.60x | 1.37x | 1.03x | 1.71x | 1.53x | 28/30 |
| perft_full_d3 | 0.85x | 0.55x | 0.66x | 0.56x | 0.76x | 0.63x | 0/30 |
| fen_parse | 1.04x | 1.52x | 1.24x | 1.09x | 1.30x | 1.05x | 29/30 |
| fen_write_native_ep | 1.04x | 2.00x | 6.60x | 1.18x | 18.86x | 1.56x | 30/30 |
| snapshot | 1.12x | 0.73x | 0.56x | 1.45x | 0.56x | 0.50x | 6/30 |
| san_parse_all | 1.59x | 1.18x | N/A | 2.61x | N/A | N/A | 11/12 |
| san_render_native | 1.29x | 4.61x | N/A | 1.06x | N/A | N/A | 10/12 |
| apply_children_full | 1.04x | 0.73x | 0.48x | 1.40x | 0.45x | 0.52x | 6/30 |
| make_unmake_full | 1.13x | N/A | N/A | 0.86x | N/A | N/A | 0/6 |
| startpos | 0.87x | 0.75x | 36.91x | 62.78x | 70.65x | 60.21x | 4/5 |
| chess960_materialize | 1.15x | 1.61x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.14x | 1.59x | 1.13x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 34.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.96x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.22x | 17.38x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.31x | 2.16x | 1.18x | 2.21x | 2.96x | 10/10 |
| corpus_materialize | 1.01x | 1.53x | 1.42x | 1.16x | 1.87x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.18x | 0.95x | 1.47x | 1.09x | 1.42x | 0.76x | 6/10 |
| replay_trusted_native | 1.06x | 0.55x | 1.02x | 1.00x | 1.37x | 1.24x | 6/10 |
| replay_checked_native | 1.09x | 1.56x | 0.87x | 6.40x | 6.29x | 7.11x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.

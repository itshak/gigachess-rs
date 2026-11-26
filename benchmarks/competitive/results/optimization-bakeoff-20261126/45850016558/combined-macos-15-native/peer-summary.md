| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.93x | 5.56x | 1.81x | 1.89x | 2.26x | 2.39x | 30/30 |
| materialize | 0.92x | 1.88x | 1.46x | 1.07x | 2.42x | 1.42x | 28/30 |
| perft_bulk_d4 | 1.09x | 3.93x | 1.50x | 1.32x | 1.97x | 2.03x | 30/30 |
| perft_full_d3 | 0.81x | 1.38x | 1.49x | 1.15x | 1.56x | 1.69x | 30/30 |
| fen_parse | 0.93x | 1.21x | 1.25x | 1.29x | 1.65x | 1.49x | 29/30 |
| fen_write_native_ep | 0.89x | 4.01x | 8.78x | 1.65x | 27.79x | 2.32x | 30/30 |
| snapshot | 0.98x | 1.60x | 1.20x | 2.54x | 1.19x | 1.03x | 26/30 |
| san_parse_all | 1.07x | 1.97x | N/A | 4.20x | N/A | N/A | 12/12 |
| san_render_native | 0.92x | 10.36x | N/A | 2.35x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.09x | 1.21x | 6.28x | 1.10x | 1.27x | 28/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 0.99x | 2.13x | 91.09x | 223.02x | 200.86x | 216.20x | 5/5 |
| chess960_materialize | 0.92x | 1.85x | 1.73x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 3.14x | 1.62x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.90x | 40.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.92x | 1.12x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.90x | 20.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.72x | 4.11x | 1.50x | 0.83x | 1.95x | 1.78x | 9/10 |
| corpus_materialize | 1.02x | 1.65x | 1.41x | 1.25x | 1.99x | 1.24x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.01x | 2.68x | 1.86x | 2.58x | 1.34x | 10/10 |
| replay_trusted_native | 0.89x | 0.59x | 1.66x | 1.58x | 1.81x | 1.79x | 8/10 |
| replay_checked_native | 0.92x | 2.26x | 1.19x | 8.65x | 7.81x | 10.18x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

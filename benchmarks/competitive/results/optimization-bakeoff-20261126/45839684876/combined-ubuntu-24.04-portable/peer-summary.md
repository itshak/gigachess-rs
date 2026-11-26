| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.19x | 1.62x | 1.46x | 2.07x | 1.45x | 30/30 |
| materialize | 0.99x | 2.16x | 1.86x | 1.36x | 2.50x | 1.61x | 30/30 |
| perft_bulk_d4 | 0.95x | 2.95x | 1.59x | 1.23x | 2.01x | 1.73x | 30/30 |
| perft_full_d3 | 0.97x | 1.17x | 1.36x | 1.10x | 1.64x | 1.50x | 29/30 |
| fen_parse | 0.86x | 1.41x | 1.42x | 1.05x | 2.02x | 1.19x | 28/30 |
| fen_write_native_ep | 1.02x | 2.92x | 9.82x | 1.79x | 26.74x | 1.97x | 30/30 |
| snapshot | 1.00x | 1.69x | 1.23x | 3.06x | 1.02x | 1.07x | 30/30 |
| san_parse_all | 1.00x | 2.04x | N/A | 4.18x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.21x | N/A | 2.22x | N/A | N/A | 12/12 |
| apply_children_full | 0.90x | 1.97x | 1.37x | 3.95x | 1.27x | 1.30x | 30/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 66.32x | 105.12x | 181.25x | 110.39x | 5/5 |
| chess960_materialize | 0.99x | 1.63x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.21x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.83x | 22.84x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.96x | 17.92x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 3.89x | 1.95x | 1.30x | 2.16x | 2.58x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.62x | 1.21x | 2.10x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.21x | 3.20x | 2.45x | 3.11x | 1.75x | 10/10 |
| replay_trusted_native | 1.00x | 0.86x | 1.65x | 1.92x | 2.56x | 2.37x | 8/10 |
| replay_checked_native | 1.00x | 2.15x | 1.05x | 7.48x | 7.66x | 8.48x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

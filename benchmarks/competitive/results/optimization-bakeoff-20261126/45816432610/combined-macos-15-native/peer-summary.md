| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.50x | 1.52x | 1.65x | 2.16x | 1.94x | 29/30 |
| materialize | 1.25x | 2.18x | 1.53x | 1.12x | 2.31x | 1.47x | 28/30 |
| perft_bulk_d4 | 0.83x | 3.60x | 1.40x | 1.14x | 1.80x | 1.76x | 28/30 |
| perft_full_d3 | 0.98x | 1.52x | 1.71x | 1.34x | 2.05x | 2.05x | 30/30 |
| fen_parse | 0.91x | 1.18x | 1.25x | 1.25x | 1.47x | 1.37x | 30/30 |
| fen_write_native_ep | 0.96x | 4.22x | 8.80x | 1.74x | 28.21x | 2.24x | 30/30 |
| snapshot | 0.85x | 1.57x | 1.07x | 2.05x | 1.23x | 0.98x | 24/30 |
| san_parse_all | 0.83x | 1.90x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 0.83x | 10.80x | N/A | 2.59x | N/A | N/A | 12/12 |
| apply_children_full | 0.86x | 1.04x | 1.18x | 5.31x | 1.10x | 1.23x | 28/30 |
| make_unmake_full | 0.85x | N/A | N/A | 1.09x | N/A | N/A | 5/6 |
| startpos | 1.10x | 1.19x | 91.32x | 177.58x | 171.17x | 159.73x | 5/5 |
| chess960_materialize | 0.93x | 1.67x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.88x | 3.06x | 1.56x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.80x | 32.88x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.05x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.79x | 4.73x | 1.87x | 1.23x | 2.08x | 2.36x | 10/10 |
| corpus_materialize | 1.02x | 1.56x | 1.45x | 1.19x | 1.93x | 1.34x | 10/10 |
| corpus_uci_sorted | 0.90x | 1.76x | 2.28x | 1.62x | 2.23x | 1.32x | 10/10 |
| replay_trusted_native | 1.03x | 0.69x | 1.68x | 1.81x | 1.93x | 2.13x | 8/10 |
| replay_checked_native | 1.10x | 2.62x | 1.15x | 8.96x | 7.99x | 9.84x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

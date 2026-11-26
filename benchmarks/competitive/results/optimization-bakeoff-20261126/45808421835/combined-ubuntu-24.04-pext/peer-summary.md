| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.28x | 1.79x | 1.55x | 2.57x | 1.82x | 30/30 |
| materialize | 1.01x | 2.12x | 1.57x | 1.65x | 2.58x | 1.96x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.04x | 1.61x | 1.26x | 2.51x | 1.96x | 30/30 |
| perft_full_d3 | 0.99x | 1.51x | 1.76x | 1.71x | 2.28x | 1.84x | 30/30 |
| fen_parse | 1.04x | 1.33x | 1.39x | 1.19x | 1.56x | 1.20x | 30/30 |
| fen_write_native_ep | 0.99x | 4.86x | 10.04x | 1.89x | 28.98x | 2.16x | 30/30 |
| snapshot | 0.91x | 1.50x | 1.17x | 2.00x | 1.17x | 1.00x | 26/30 |
| san_parse_all | 1.03x | 2.17x | N/A | 4.57x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.48x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.96x | 1.29x | 3.50x | 1.28x | 2.09x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.11x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 68.33x | 121.02x | 148.95x | 121.40x | 5/5 |
| chess960_materialize | 0.99x | 1.74x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.36x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.98x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.41x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.86x | 2.55x | 1.40x | 3.14x | 2.86x | 10/10 |
| corpus_materialize | 1.01x | 1.48x | 1.34x | 1.12x | 1.80x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.33x | 3.08x | 2.24x | 3.05x | 1.60x | 10/10 |
| replay_trusted_native | 1.00x | 0.80x | 1.47x | 1.48x | 2.08x | 2.33x | 9/10 |
| replay_checked_native | 0.99x | 2.08x | 1.10x | 8.78x | 8.49x | 10.00x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.37x | 1.67x | 1.80x | 2.17x | 2.16x | 30/30 |
| materialize | 1.00x | 1.94x | 1.54x | 1.34x | 2.14x | 1.48x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.80x | 1.57x | 1.41x | 2.01x | 2.04x | 30/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.60x | 1.33x | 1.99x | 1.86x | 30/30 |
| fen_parse | 1.00x | 1.23x | 1.32x | 1.27x | 1.74x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 4.52x | 9.39x | 1.57x | 30.06x | 2.39x | 30/30 |
| snapshot | 0.99x | 1.07x | 1.02x | 1.47x | 1.01x | 0.95x | 20/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.21x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.34x | N/A | 2.68x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 1.18x | 1.40x | 3.82x | 1.14x | 1.14x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.50x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.62x | 130.03x | 171.80x | 139.74x | 5/5 |
| chess960_materialize | 1.00x | 1.70x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.97x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.67x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.24x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.38x | 1.70x | 1.35x | 1.98x | 2.20x | 10/10 |
| corpus_materialize | 0.99x | 1.58x | 1.52x | 1.23x | 1.89x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.92x | 2.07x | 2.85x | 1.45x | 10/10 |
| replay_trusted_native | 0.91x | 0.58x | 1.52x | 1.84x | 2.05x | 2.08x | 8/10 |
| replay_checked_native | 0.96x | 2.21x | 1.08x | 7.28x | 6.53x | 8.42x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

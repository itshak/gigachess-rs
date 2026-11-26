| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.14x | 1.81x | 1.55x | 2.45x | 1.96x | 30/30 |
| materialize | 0.99x | 2.05x | 1.55x | 1.64x | 2.41x | 1.82x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.68x | 1.50x | 1.15x | 2.25x | 1.82x | 29/30 |
| perft_full_d3 | 1.00x | 1.41x | 1.60x | 1.49x | 2.00x | 2.19x | 30/30 |
| fen_parse | 0.99x | 1.18x | 1.49x | 1.19x | 1.61x | 1.24x | 30/30 |
| fen_write_native_ep | 1.00x | 4.86x | 10.41x | 2.12x | 30.04x | 2.24x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.67x | 1.00x | 1.33x | 30/30 |
| san_parse_all | 1.07x | 1.89x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 8.79x | N/A | 2.04x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 2.01x | 1.27x | 3.85x | 1.37x | 1.56x | 30/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.08x | N/A | N/A | 5/6 |
| startpos | 0.89x | 1.00x | 81.54x | 142.37x | 182.95x | 143.59x | 4/5 |
| chess960_materialize | 0.97x | 1.52x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.48x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.58x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.80x | 24.10x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.70x | 1.85x | 0.95x | 2.24x | 2.03x | 8/10 |
| corpus_materialize | 0.99x | 1.38x | 1.28x | 1.03x | 1.66x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.28x | 3.01x | 2.29x | 2.91x | 1.56x | 10/10 |
| replay_trusted_native | 1.02x | 0.87x | 1.53x | 1.57x | 2.18x | 2.47x | 9/10 |
| replay_checked_native | 0.97x | 1.96x | 1.04x | 8.43x | 8.23x | 9.66x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.00x | 1.68x | 1.46x | 2.35x | 1.69x | 30/30 |
| materialize | 1.00x | 2.13x | 1.60x | 1.71x | 2.57x | 1.86x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.42x | 1.36x | 1.06x | 2.24x | 1.66x | 29/30 |
| perft_full_d3 | 1.01x | 1.46x | 1.71x | 1.66x | 2.12x | 1.79x | 30/30 |
| fen_parse | 1.00x | 1.36x | 1.47x | 1.06x | 1.37x | 1.06x | 30/30 |
| fen_write_native_ep | 0.96x | 4.51x | 9.59x | 2.06x | 26.60x | 2.42x | 30/30 |
| snapshot | 1.00x | 1.82x | 1.16x | 7.36x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.93x | N/A | 4.44x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.16x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.87x | 1.19x | 3.71x | 1.18x | 1.50x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 1.11x | 1.12x | 90.17x | 166.68x | 200.78x | 162.18x | 5/5 |
| chess960_materialize | 1.08x | 1.73x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.39x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.54x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.04x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.04x | 16.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.45x | 1.64x | 0.92x | 2.04x | 1.82x | 8/10 |
| corpus_materialize | 1.01x | 1.38x | 1.28x | 1.07x | 1.70x | 1.14x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.26x | 2.96x | 2.23x | 2.82x | 1.56x | 10/10 |
| replay_trusted_native | 0.99x | 0.83x | 1.48x | 1.51x | 2.02x | 2.32x | 9/10 |
| replay_checked_native | 1.00x | 2.10x | 1.12x | 8.83x | 8.35x | 9.81x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

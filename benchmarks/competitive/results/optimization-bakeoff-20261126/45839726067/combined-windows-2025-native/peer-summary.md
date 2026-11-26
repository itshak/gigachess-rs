| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.89x | 1.73x | 1.65x | 2.39x | 2.15x | 30/30 |
| materialize | 1.06x | 2.05x | 1.38x | 1.28x | 2.38x | 1.59x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.10x | 1.64x | 1.37x | 2.30x | 2.09x | 30/30 |
| perft_full_d3 | 1.06x | 1.16x | 1.23x | 1.10x | 1.60x | 1.54x | 30/30 |
| fen_parse | 1.33x | 1.48x | 1.63x | 1.48x | 2.52x | 1.65x | 30/30 |
| fen_write_native_ep | 1.06x | 4.13x | 8.46x | 1.57x | 32.35x | 1.99x | 30/30 |
| snapshot | 1.03x | 1.83x | 1.01x | 4.26x | 1.00x | 1.03x | 27/30 |
| san_parse_all | 1.01x | 2.05x | N/A | 5.57x | N/A | N/A | 12/12 |
| san_render_native | 1.07x | 11.32x | N/A | 3.96x | N/A | N/A | 12/12 |
| apply_children_full | 1.07x | 2.31x | 1.40x | 9.58x | 1.48x | 1.67x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.69x | 154.31x | 236.55x | 170.91x | 5/5 |
| chess960_materialize | 1.24x | 1.68x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.88x | 1.54x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 28.67x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.11x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 22.13x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 5.34x | 1.83x | 1.34x | 2.45x | 2.42x | 10/10 |
| corpus_materialize | 1.11x | 1.56x | 1.26x | 1.15x | 2.00x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.04x | 1.73x | 2.16x | 1.78x | 2.12x | 1.39x | 10/10 |
| replay_trusted_native | 0.99x | 0.82x | 1.56x | 2.04x | 2.55x | 2.88x | 9/10 |
| replay_checked_native | 1.21x | 2.57x | 1.11x | 7.86x | 7.98x | 9.37x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

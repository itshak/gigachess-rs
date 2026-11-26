| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.52x | 1.57x | 1.59x | 2.48x | 1.82x | 30/30 |
| materialize | 1.01x | 2.21x | 1.70x | 1.16x | 2.17x | 1.44x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.93x | 1.52x | 1.27x | 2.44x | 1.99x | 30/30 |
| perft_full_d3 | 1.07x | 1.32x | 1.36x | 1.04x | 2.02x | 1.57x | 26/30 |
| fen_parse | 0.99x | 1.17x | 1.30x | 0.99x | 1.64x | 1.14x | 26/30 |
| fen_write_native_ep | 1.01x | 4.13x | 9.64x | 1.82x | 27.87x | 2.17x | 30/30 |
| snapshot | 1.00x | 1.91x | 1.25x | 3.84x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.38x | N/A | 3.89x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 10.96x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.94x | 1.25x | 4.73x | 1.45x | 1.63x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.20x | 100.03x | 138.75x | 211.96x | 157.69x | 5/5 |
| chess960_materialize | 1.01x | 1.81x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.91x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.84x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.22x | 19.06x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.22x | 1.62x | 1.25x | 2.28x | 1.89x | 9/10 |
| corpus_materialize | 1.00x | 1.92x | 1.72x | 1.23x | 2.83x | 1.50x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.31x | 3.12x | 2.33x | 2.93x | 1.66x | 10/10 |
| replay_trusted_native | 1.05x | 1.15x | 2.00x | 2.24x | 3.82x | 4.56x | 10/10 |
| replay_checked_native | 1.00x | 2.69x | 1.06x | 5.90x | 6.73x | 7.79x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.69x | 1.70x | 1.64x | 2.49x | 2.03x | 30/30 |
| materialize | 0.99x | 2.06x | 1.72x | 1.29x | 2.35x | 1.55x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.01x | 1.60x | 1.36x | 2.34x | 2.01x | 30/30 |
| perft_full_d3 | 0.99x | 1.36x | 1.50x | 1.33x | 1.92x | 2.09x | 30/30 |
| fen_parse | 1.04x | 1.41x | 1.43x | 1.24x | 2.22x | 1.38x | 30/30 |
| fen_write_native_ep | 0.98x | 3.97x | 9.16x | 1.71x | 27.68x | 1.98x | 30/30 |
| snapshot | 0.46x | 1.08x | 0.93x | 1.65x | 1.12x | 0.70x | 18/30 |
| san_parse_all | 0.99x | 2.00x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 10.00x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.34x | 1.35x | 4.47x | 1.48x | 1.71x | 30/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.33x | 80.39x | 148.35x | 219.77x | 151.14x | 5/5 |
| chess960_materialize | 0.98x | 1.66x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.83x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.33x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.94x | 18.22x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.36x | 1.81x | 1.36x | 2.60x | 2.39x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.56x | 1.19x | 2.06x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.40x | 3.19x | 2.43x | 3.05x | 1.74x | 10/10 |
| replay_trusted_native | 1.01x | 0.87x | 1.61x | 1.92x | 2.55x | 2.90x | 9/10 |
| replay_checked_native | 1.01x | 2.30x | 1.03x | 8.17x | 7.74x | 9.20x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

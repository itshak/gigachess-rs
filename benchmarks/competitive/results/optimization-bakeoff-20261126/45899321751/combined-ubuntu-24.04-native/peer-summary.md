| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.27x | 1.81x | 1.56x | 2.19x | 1.96x | 30/30 |
| materialize | 0.99x | 2.02x | 1.56x | 1.63x | 2.38x | 1.62x | 30/30 |
| perft_bulk_d4 | 1.05x | 3.96x | 1.60x | 1.23x | 2.24x | 1.94x | 29/30 |
| perft_full_d3 | 1.00x | 1.36x | 1.59x | 1.46x | 1.97x | 1.71x | 30/30 |
| fen_parse | 0.98x | 1.30x | 1.56x | 1.16x | 1.51x | 1.31x | 30/30 |
| fen_write_native_ep | 1.00x | 5.42x | 10.42x | 1.92x | 28.55x | 2.63x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.01x | 4.69x | 1.00x | 1.33x | 26/30 |
| san_parse_all | 0.93x | 1.83x | N/A | 3.84x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.29x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.15x | 1.36x | 4.12x | 1.44x | 1.66x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 0.90x | 1.00x | 81.12x | 138.11x | 177.93x | 152.76x | 4/5 |
| chess960_materialize | 1.03x | 1.62x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.53x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.43x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 27.06x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.39x | 6.46x | 2.53x | 1.28x | 2.90x | 2.81x | 10/10 |
| corpus_materialize | 1.02x | 1.40x | 1.29x | 1.06x | 1.69x | 1.17x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.36x | 2.98x | 2.23x | 2.87x | 1.55x | 10/10 |
| replay_trusted_native | 0.99x | 0.84x | 1.53x | 1.48x | 2.06x | 2.62x | 9/10 |
| replay_checked_native | 1.03x | 2.43x | 1.27x | 9.48x | 9.14x | 10.98x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

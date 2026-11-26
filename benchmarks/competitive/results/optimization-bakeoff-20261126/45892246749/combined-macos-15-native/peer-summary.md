| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.82x | 4.57x | 1.54x | 1.63x | 2.21x | 1.93x | 29/30 |
| materialize | 0.96x | 2.12x | 1.61x | 1.03x | 2.20x | 1.33x | 28/30 |
| perft_bulk_d4 | 0.84x | 3.54x | 1.38x | 1.07x | 1.74x | 1.71x | 28/30 |
| perft_full_d3 | 0.72x | 1.20x | 1.33x | 1.01x | 1.54x | 1.61x | 27/30 |
| fen_parse | 0.82x | 1.17x | 1.19x | 1.22x | 1.48x | 1.39x | 30/30 |
| fen_write_native_ep | 0.93x | 3.99x | 8.76x | 1.65x | 28.67x | 2.12x | 30/30 |
| snapshot | 1.13x | 1.95x | 1.28x | 2.70x | 1.28x | 1.18x | 30/30 |
| san_parse_all | 0.96x | 1.72x | N/A | 3.57x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.59x | N/A | 2.93x | N/A | N/A | 12/12 |
| apply_children_full | 0.43x | 0.54x | 0.57x | 2.69x | 0.54x | 0.61x | 6/30 |
| make_unmake_full | 0.64x | N/A | N/A | 0.91x | N/A | N/A | 1/6 |
| startpos | 2.11x | 2.39x | 135.90x | 277.72x | 320.10x | 298.88x | 5/5 |
| chess960_materialize | 1.23x | 1.83x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.74x | 1.24x | N/A | N/A | N/A | 3/4 |
| polyglot_key | 1.12x | 40.88x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.06x | 21.81x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 5.41x | 1.70x | 1.07x | 2.02x | 2.23x | 9/10 |
| corpus_materialize | 1.07x | 1.54x | 1.41x | 1.13x | 1.90x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.80x | 2.73x | 1.90x | 2.41x | 1.39x | 10/10 |
| replay_trusted_native | 0.84x | 0.61x | 1.44x | 1.49x | 1.57x | 1.59x | 8/10 |
| replay_checked_native | 0.91x | 2.43x | 1.17x | 9.18x | 7.56x | 9.65x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.75x | 1.69x | 1.61x | 2.60x | 1.96x | 30/30 |
| materialize | 0.95x | 2.13x | 1.67x | 1.13x | 2.18x | 1.51x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.85x | 1.51x | 1.23x | 2.35x | 2.03x | 30/30 |
| perft_full_d3 | 0.71x | 0.83x | 0.87x | 0.66x | 1.19x | 1.02x | 8/30 |
| fen_parse | 0.85x | 1.29x | 1.36x | 1.12x | 2.08x | 1.16x | 29/30 |
| fen_write_native_ep | 0.96x | 3.86x | 8.81x | 1.73x | 26.44x | 2.18x | 30/30 |
| snapshot | 0.73x | 1.59x | 1.09x | 2.68x | 1.09x | 1.00x | 26/30 |
| san_parse_all | 0.87x | 1.94x | N/A | 3.38x | N/A | N/A | 12/12 |
| san_render_native | 0.92x | 10.06x | N/A | 1.93x | N/A | N/A | 12/12 |
| apply_children_full | 0.54x | 1.11x | 0.75x | 2.64x | 0.82x | 0.97x | 14/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.12x | N/A | N/A | 4/6 |
| startpos | 0.75x | 0.97x | 73.12x | 113.51x | 185.66x | 115.42x | 4/5 |
| chess960_materialize | 0.91x | 1.69x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.87x | 2.70x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 21.24x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 18.69x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.48x | 1.74x | 1.27x | 2.32x | 1.99x | 9/10 |
| corpus_materialize | 0.99x | 2.02x | 1.85x | 1.27x | 2.95x | 1.59x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.28x | 3.07x | 2.30x | 2.86x | 1.64x | 10/10 |
| replay_trusted_native | 0.87x | 0.99x | 1.65x | 1.92x | 3.46x | 4.29x | 8/10 |
| replay_checked_native | 0.90x | 2.39x | 0.91x | 5.28x | 6.22x | 7.24x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

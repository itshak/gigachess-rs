| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 4.50x | 1.93x | 1.64x | 2.52x | 1.98x | 30/30 |
| materialize | 0.99x | 2.21x | 1.67x | 1.73x | 2.61x | 1.91x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.96x | 1.62x | 1.23x | 2.42x | 1.95x | 29/30 |
| perft_full_d3 | 1.24x | 1.74x | 1.96x | 1.84x | 2.44x | 2.04x | 30/30 |
| fen_parse | 0.98x | 1.39x | 1.36x | 1.06x | 1.46x | 1.25x | 30/30 |
| fen_write_native_ep | 0.96x | 4.74x | 9.69x | 2.06x | 27.43x | 2.56x | 30/30 |
| snapshot | 0.99x | 1.81x | 1.15x | 4.29x | 1.15x | 1.32x | 30/30 |
| san_parse_all | 0.90x | 1.71x | N/A | 3.89x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.05x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.18x | 1.37x | 4.13x | 1.38x | 1.73x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.12x | 88.39x | 153.22x | 191.46x | 174.40x | 5/5 |
| chess960_materialize | 0.99x | 1.73x | 1.72x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.54x | 1.54x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 38.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.61x | 1.62x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.23x | 6.51x | 2.43x | 1.34x | 3.08x | 2.68x | 10/10 |
| corpus_materialize | 1.02x | 1.41x | 1.29x | 1.07x | 1.72x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.05x | 2.34x | 3.00x | 2.25x | 2.94x | 1.59x | 10/10 |
| replay_trusted_native | 1.00x | 0.87x | 1.60x | 1.51x | 2.14x | 2.54x | 9/10 |
| replay_checked_native | 0.99x | 2.19x | 1.14x | 9.06x | 8.90x | 10.47x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

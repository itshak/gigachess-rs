| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.82x | 4.67x | 1.50x | 1.77x | 2.30x | 1.93x | 29/30 |
| materialize | 0.78x | 1.83x | 1.48x | 0.99x | 2.06x | 1.37x | 27/30 |
| perft_bulk_d4 | 0.89x | 4.10x | 1.52x | 1.20x | 1.87x | 1.83x | 29/30 |
| perft_full_d3 | 0.82x | 1.47x | 1.68x | 1.26x | 1.89x | 1.85x | 29/30 |
| fen_parse | 0.92x | 1.32x | 1.61x | 1.41x | 1.99x | 1.42x | 30/30 |
| fen_write_native_ep | 0.83x | 4.12x | 12.84x | 1.60x | 24.23x | 1.87x | 30/30 |
| snapshot | 0.84x | 1.60x | 0.97x | 2.30x | 0.90x | 0.85x | 18/30 |
| san_parse_all | 0.87x | 1.80x | N/A | 3.66x | N/A | N/A | 12/12 |
| san_render_native | 0.83x | 11.82x | N/A | 2.98x | N/A | N/A | 12/12 |
| apply_children_full | 0.68x | 0.90x | 0.97x | 4.90x | 0.94x | 1.12x | 14/30 |
| make_unmake_full | 0.77x | N/A | N/A | 1.28x | N/A | N/A | 5/6 |
| startpos | 0.68x | 1.08x | 61.99x | 123.96x | 155.39x | 123.01x | 5/5 |
| chess960_materialize | 0.89x | 1.68x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.86x | 3.59x | 1.72x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.90x | 35.14x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.93x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.93x | 20.72x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.05x | 5.81x | 2.16x | 1.05x | 2.48x | 2.54x | 9/10 |
| corpus_materialize | 1.02x | 1.35x | 1.38x | 1.06x | 1.82x | 1.30x | 9/10 |
| corpus_uci_sorted | 1.18x | 2.03x | 2.91x | 2.06x | 2.67x | 1.38x | 10/10 |
| replay_trusted_native | 1.11x | 0.80x | 1.73x | 1.71x | 2.09x | 2.02x | 8/10 |
| replay_checked_native | 1.06x | 2.99x | 1.53x | 11.38x | 9.49x | 10.97x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

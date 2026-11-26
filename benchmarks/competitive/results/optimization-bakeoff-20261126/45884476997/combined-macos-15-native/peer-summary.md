| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 5.12x | 1.69x | 1.82x | 2.42x | 2.25x | 30/30 |
| materialize | 0.92x | 1.98x | 1.58x | 1.07x | 2.24x | 1.35x | 27/30 |
| perft_bulk_d4 | 0.93x | 4.12x | 1.53x | 1.32x | 2.02x | 1.93x | 29/30 |
| perft_full_d3 | 1.05x | 1.84x | 2.15x | 1.53x | 2.28x | 2.34x | 30/30 |
| fen_parse | 0.96x | 1.25x | 1.25x | 1.27x | 1.61x | 1.67x | 30/30 |
| fen_write_native_ep | 1.00x | 4.14x | 9.16x | 1.78x | 27.80x | 2.21x | 30/30 |
| snapshot | 0.99x | 1.60x | 1.15x | 2.39x | 1.16x | 0.96x | 26/30 |
| san_parse_all | 1.03x | 1.78x | N/A | 3.75x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 12.54x | N/A | 2.79x | N/A | N/A | 12/12 |
| apply_children_full | 1.14x | 1.16x | 1.22x | 5.98x | 1.20x | 1.34x | 29/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.04x | 1.18x | 75.82x | 148.90x | 169.76x | 181.59x | 5/5 |
| chess960_materialize | 1.29x | 1.74x | 1.62x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 2.91x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.18x | 38.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.12x | 1.02x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 20.38x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.34x | 1.69x | 1.18x | 1.94x | 2.39x | 10/10 |
| corpus_materialize | 0.86x | 1.34x | 1.37x | 1.14x | 1.92x | 1.17x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.93x | 2.73x | 1.90x | 2.44x | 1.38x | 10/10 |
| replay_trusted_native | 0.95x | 0.61x | 1.53x | 1.58x | 1.73x | 1.72x | 8/10 |
| replay_checked_native | 0.88x | 2.39x | 1.13x | 7.99x | 6.82x | 8.94x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

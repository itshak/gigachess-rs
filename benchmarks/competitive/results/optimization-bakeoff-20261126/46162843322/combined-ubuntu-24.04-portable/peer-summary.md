| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 3.05x | 1.74x | 1.52x | 2.03x | 1.45x | 30/30 |
| materialize | 1.00x | 2.40x | 1.88x | 1.87x | 2.95x | 2.01x | 30/30 |
| perft_bulk_d4 | 1.07x | 3.21x | 1.72x | 1.28x | 2.14x | 1.90x | 30/30 |
| perft_full_d3 | 1.14x | 1.56x | 1.80x | 1.64x | 2.17x | 1.81x | 30/30 |
| fen_parse | 0.94x | 1.60x | 1.34x | 1.09x | 1.50x | 1.12x | 30/30 |
| fen_write_native_ep | 1.00x | 3.10x | 9.96x | 1.96x | 26.77x | 2.30x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 28/30 |
| san_parse_all | 0.99x | 1.91x | N/A | 4.44x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.11x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.17x | 1.43x | 4.08x | 1.33x | 1.55x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.35x | 57.14x | 108.78x | 131.61x | 105.82x | 5/5 |
| chess960_materialize | 0.98x | 1.69x | 1.65x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.15x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 44.00x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.64x | 1.63x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.08x | 26.37x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.14x | 4.87x | 2.46x | 1.29x | 2.48x | 3.34x | 10/10 |
| corpus_materialize | 1.01x | 1.51x | 1.41x | 1.15x | 1.86x | 1.25x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.95x | 2.89x | 2.22x | 2.90x | 1.54x | 10/10 |
| replay_trusted_native | 0.94x | 0.86x | 1.56x | 1.54x | 2.17x | 1.99x | 9/10 |
| replay_checked_native | 0.90x | 1.88x | 1.05x | 8.08x | 7.95x | 8.98x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

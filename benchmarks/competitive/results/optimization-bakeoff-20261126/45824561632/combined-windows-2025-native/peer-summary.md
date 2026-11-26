| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.27x | 1.55x | 1.50x | 2.29x | 1.90x | 30/30 |
| materialize | 0.96x | 1.94x | 1.29x | 1.08x | 2.16x | 1.40x | 28/30 |
| perft_bulk_d4 | 1.03x | 3.66x | 1.48x | 1.32x | 2.24x | 2.08x | 30/30 |
| perft_full_d3 | 0.86x | 1.05x | 1.06x | 0.90x | 1.47x | 1.33x | 21/30 |
| fen_parse | 1.03x | 1.39x | 1.44x | 1.34x | 2.21x | 1.48x | 30/30 |
| fen_write_native_ep | 1.01x | 3.98x | 7.70x | 1.63x | 32.34x | 2.04x | 30/30 |
| snapshot | 0.89x | 1.92x | 1.12x | 2.83x | 1.12x | 0.97x | 25/30 |
| san_parse_all | 0.93x | 1.82x | N/A | 4.54x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.41x | N/A | 3.39x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.87x | 1.25x | 7.29x | 1.51x | 1.55x | 30/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 91.61x | 157.45x | 245.73x | 178.47x | 5/5 |
| chess960_materialize | 0.94x | 1.54x | 1.22x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.60x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 21.06x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.23x | 19.57x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.03x | 1.61x | 1.17x | 2.13x | 1.92x | 9/10 |
| corpus_materialize | 1.01x | 1.70x | 1.34x | 1.14x | 2.71x | 1.46x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.88x | 2.37x | 1.97x | 2.26x | 1.51x | 10/10 |
| replay_trusted_native | 0.78x | 0.92x | 1.58x | 1.97x | 3.15x | 4.02x | 9/10 |
| replay_checked_native | 0.92x | 2.11x | 0.92x | 5.24x | 6.20x | 7.37x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

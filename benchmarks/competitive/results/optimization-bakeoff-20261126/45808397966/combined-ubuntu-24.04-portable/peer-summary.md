| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 3.03x | 1.53x | 1.39x | 2.08x | 1.38x | 30/30 |
| materialize | 0.85x | 1.69x | 1.50x | 1.07x | 2.03x | 1.29x | 27/30 |
| perft_bulk_d4 | 1.01x | 2.92x | 1.60x | 1.23x | 2.07x | 1.73x | 30/30 |
| perft_full_d3 | 0.99x | 1.45x | 1.66x | 1.35x | 1.99x | 1.79x | 30/30 |
| fen_parse | 0.99x | 1.56x | 1.48x | 1.17x | 2.27x | 1.34x | 30/30 |
| fen_write_native_ep | 1.00x | 2.87x | 10.29x | 1.72x | 27.85x | 1.99x | 30/30 |
| snapshot | 1.00x | 1.69x | 1.23x | 3.08x | 1.02x | 1.07x | 29/30 |
| san_parse_all | 1.02x | 2.04x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 0.94x | 10.56x | N/A | 2.36x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.12x | 1.45x | 4.29x | 1.35x | 1.28x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.06x | 1.34x | 66.60x | 107.58x | 185.42x | 119.07x | 5/5 |
| chess960_materialize | 0.95x | 1.41x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.08x | 2.22x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 27.59x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.86x | 18.20x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 3.66x | 1.81x | 1.21x | 2.00x | 2.42x | 10/10 |
| corpus_materialize | 0.88x | 1.45x | 1.48x | 1.06x | 1.90x | 1.24x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.16x | 3.24x | 2.41x | 3.11x | 1.77x | 10/10 |
| replay_trusted_native | 1.00x | 0.78x | 1.55x | 1.74x | 2.32x | 2.15x | 8/10 |
| replay_checked_native | 1.00x | 2.09x | 1.05x | 7.29x | 7.44x | 8.21x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

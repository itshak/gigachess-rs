| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 3.29x | 1.75x | 1.57x | 2.17x | 1.60x | 30/30 |
| materialize | 1.05x | 2.33x | 1.83x | 1.32x | 2.45x | 1.68x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.11x | 1.71x | 1.36x | 2.12x | 1.85x | 30/30 |
| perft_full_d3 | 1.23x | 1.71x | 2.07x | 1.60x | 2.44x | 2.24x | 30/30 |
| fen_parse | 0.86x | 1.31x | 1.27x | 1.07x | 1.81x | 1.12x | 30/30 |
| fen_write_native_ep | 1.03x | 2.62x | 9.29x | 1.69x | 25.60x | 2.07x | 30/30 |
| snapshot | 1.00x | 1.76x | 1.64x | 2.90x | 1.14x | 1.13x | 30/30 |
| san_parse_all | 1.03x | 1.99x | N/A | 3.97x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 9.94x | N/A | 2.28x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.21x | 1.47x | 5.78x | 1.43x | 1.44x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.39x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 72.39x | 108.51x | 168.75x | 110.76x | 5/5 |
| chess960_materialize | 1.00x | 1.72x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 2.30x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 22.04x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 21.03x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.15x | 3.32x | 1.91x | 1.33x | 2.06x | 2.00x | 10/10 |
| corpus_materialize | 1.03x | 1.89x | 1.80x | 1.32x | 2.95x | 1.65x | 10/10 |
| corpus_uci_sorted | 1.04x | 2.25x | 3.20x | 2.48x | 3.09x | 1.82x | 10/10 |
| replay_trusted_native | 1.04x | 1.07x | 2.20x | 2.33x | 3.51x | 3.12x | 10/10 |
| replay_checked_native | 1.07x | 2.34x | 1.18x | 6.35x | 7.59x | 7.87x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

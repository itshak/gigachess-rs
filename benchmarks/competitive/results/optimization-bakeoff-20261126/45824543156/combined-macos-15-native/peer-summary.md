| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 4.90x | 1.66x | 1.63x | 2.10x | 1.86x | 30/30 |
| materialize | 0.96x | 1.87x | 1.30x | 0.94x | 1.98x | 1.24x | 25/30 |
| perft_bulk_d4 | 1.05x | 3.67x | 1.38x | 1.32x | 1.75x | 1.78x | 30/30 |
| perft_full_d3 | 0.93x | 1.56x | 1.69x | 1.30x | 1.85x | 1.91x | 30/30 |
| fen_parse | 1.12x | 1.40x | 1.43x | 1.55x | 1.89x | 1.51x | 30/30 |
| fen_write_native_ep | 0.88x | 3.95x | 7.76x | 1.58x | 24.38x | 2.03x | 30/30 |
| snapshot | 1.00x | 2.07x | 1.20x | 2.57x | 1.18x | 1.15x | 25/30 |
| san_parse_all | 1.02x | 1.79x | N/A | 3.96x | N/A | N/A | 12/12 |
| san_render_native | 0.94x | 11.61x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 1.11x | 1.31x | 5.89x | 1.13x | 1.23x | 26/30 |
| make_unmake_full | 1.09x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 0.59x | 0.87x | 55.72x | 110.83x | 122.38x | 100.84x | 4/5 |
| chess960_materialize | 1.12x | 1.59x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.14x | 2.82x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.15x | 37.60x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.16x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.20x | 24.38x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 4.95x | 1.64x | 0.97x | 2.04x | 2.15x | 9/10 |
| corpus_materialize | 1.19x | 1.41x | 1.43x | 1.15x | 1.85x | 1.28x | 10/10 |
| corpus_uci_sorted | 0.97x | 1.87x | 2.41x | 1.85x | 2.26x | 1.36x | 10/10 |
| replay_trusted_native | 1.05x | 0.68x | 1.72x | 1.75x | 1.90x | 1.98x | 8/10 |
| replay_checked_native | 0.88x | 2.24x | 0.98x | 7.91x | 6.54x | 8.89x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

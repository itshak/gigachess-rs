| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.93x | 2.73x | 1.53x | 1.33x | 1.78x | 1.27x | 29/30 |
| materialize | 1.05x | 2.34x | 1.79x | 1.78x | 2.72x | 1.95x | 30/30 |
| perft_bulk_d4 | 0.86x | 2.59x | 1.38x | 1.02x | 1.71x | 1.53x | 28/30 |
| perft_full_d3 | 0.94x | 1.27x | 1.54x | 1.35x | 1.82x | 1.54x | 30/30 |
| fen_parse | 0.97x | 1.52x | 1.30x | 1.04x | 1.35x | 1.07x | 26/30 |
| fen_write_native_ep | 0.94x | 2.79x | 9.47x | 1.70x | 26.57x | 2.15x | 30/30 |
| snapshot | 1.00x | 1.63x | 1.25x | 3.38x | 1.25x | 1.12x | 30/30 |
| san_parse_all | 0.95x | 1.98x | N/A | 4.03x | N/A | N/A | 12/12 |
| san_render_native | 0.94x | 8.94x | N/A | 2.06x | N/A | N/A | 12/12 |
| apply_children_full | 0.91x | 2.05x | 1.34x | 3.89x | 1.27x | 1.45x | 30/30 |
| make_unmake_full | 0.79x | N/A | N/A | 0.97x | N/A | N/A | 2/6 |
| startpos | 1.40x | 1.78x | 79.57x | 146.93x | 171.13x | 145.01x | 5/5 |
| chess960_materialize | 0.94x | 1.69x | 1.64x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.84x | 1.80x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 39.28x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.94x | 19.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 3.94x | 1.99x | 1.08x | 2.00x | 2.72x | 10/10 |
| corpus_materialize | 0.96x | 1.45x | 1.36x | 1.10x | 1.77x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.94x | 3.06x | 2.24x | 3.04x | 1.54x | 10/10 |
| replay_trusted_native | 0.97x | 0.84x | 1.63x | 1.58x | 2.19x | 2.00x | 8/10 |
| replay_checked_native | 0.92x | 1.89x | 1.06x | 8.11x | 8.05x | 9.10x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

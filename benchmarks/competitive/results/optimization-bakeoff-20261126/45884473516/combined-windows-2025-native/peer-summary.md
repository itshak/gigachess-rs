| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.77x | 1.70x | 1.65x | 2.64x | 2.12x | 30/30 |
| materialize | 0.95x | 1.94x | 1.29x | 1.13x | 2.32x | 1.53x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.98x | 1.60x | 1.42x | 2.45x | 2.38x | 30/30 |
| perft_full_d3 | 1.00x | 1.52x | 1.58x | 1.31x | 2.19x | 1.91x | 30/30 |
| fen_parse | 0.91x | 1.28x | 1.41x | 1.36x | 2.44x | 1.42x | 30/30 |
| fen_write_native_ep | 0.99x | 4.00x | 7.85x | 1.62x | 31.87x | 2.09x | 30/30 |
| snapshot | 0.97x | 1.84x | 1.25x | 4.06x | 1.22x | 1.08x | 29/30 |
| san_parse_all | 0.97x | 1.94x | N/A | 5.05x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 9.76x | N/A | 3.47x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.95x | 1.32x | 8.41x | 1.56x | 1.74x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.22x | 83.97x | 133.81x | 206.86x | 147.82x | 5/5 |
| chess960_materialize | 1.00x | 1.67x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.92x | 1.60x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 20.38x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.06x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 19.14x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.38x | 1.76x | 1.30x | 2.38x | 2.12x | 9/10 |
| corpus_materialize | 1.01x | 1.77x | 1.40x | 1.19x | 2.84x | 1.52x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.89x | 2.40x | 1.98x | 2.28x | 1.51x | 10/10 |
| replay_trusted_native | 0.89x | 0.94x | 1.63x | 2.03x | 3.22x | 4.02x | 9/10 |
| replay_checked_native | 0.97x | 2.38x | 1.04x | 6.18x | 7.11x | 7.92x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

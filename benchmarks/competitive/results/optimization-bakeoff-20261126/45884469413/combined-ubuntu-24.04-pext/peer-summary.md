| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.64x | 1.92x | 1.68x | 2.71x | 1.96x | 30/30 |
| materialize | 1.08x | 2.30x | 1.69x | 1.81x | 2.91x | 2.01x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.86x | 1.53x | 1.21x | 2.49x | 1.87x | 29/30 |
| perft_full_d3 | 1.01x | 1.35x | 1.54x | 1.55x | 1.97x | 1.63x | 30/30 |
| fen_parse | 0.94x | 1.40x | 1.23x | 1.07x | 1.35x | 1.19x | 29/30 |
| fen_write_native_ep | 0.98x | 4.82x | 9.78x | 1.97x | 28.28x | 2.47x | 30/30 |
| snapshot | 1.00x | 1.82x | 1.15x | 4.30x | 1.15x | 1.32x | 30/30 |
| san_parse_all | 1.02x | 1.91x | N/A | 4.38x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.07x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.13x | 1.34x | 4.04x | 1.34x | 1.69x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.12x | 1.12x | 88.94x | 156.87x | 192.53x | 183.25x | 5/5 |
| chess960_materialize | 1.01x | 1.83x | 1.72x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.53x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 38.53x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.94x | 22.21x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 5.18x | 1.93x | 1.07x | 2.35x | 2.18x | 10/10 |
| corpus_materialize | 1.02x | 1.44x | 1.32x | 1.09x | 1.77x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.03x | 2.32x | 3.09x | 2.25x | 3.04x | 1.57x | 10/10 |
| replay_trusted_native | 1.03x | 0.91x | 1.65x | 1.67x | 2.27x | 2.67x | 9/10 |
| replay_checked_native | 1.00x | 2.17x | 1.16x | 9.15x | 8.96x | 10.44x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

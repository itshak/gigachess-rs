| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.31x | 1.86x | 1.58x | 2.42x | 1.91x | 30/30 |
| materialize | 0.98x | 2.18x | 1.65x | 1.69x | 2.58x | 1.91x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.76x | 1.54x | 1.17x | 2.31x | 1.85x | 29/30 |
| perft_full_d3 | 1.18x | 1.68x | 1.83x | 1.75x | 2.32x | 1.97x | 30/30 |
| fen_parse | 1.00x | 1.27x | 1.33x | 1.16x | 1.52x | 1.25x | 30/30 |
| fen_write_native_ep | 1.00x | 4.95x | 10.22x | 1.92x | 29.07x | 2.19x | 30/30 |
| snapshot | 1.00x | 1.82x | 1.15x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.04x | 1.97x | N/A | 4.42x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.02x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.16x | 1.36x | 4.08x | 1.36x | 1.75x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.12x | 87.97x | 158.60x | 202.57x | 178.28x | 5/5 |
| chess960_materialize | 1.01x | 1.77x | 1.72x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.52x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 39.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.07x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 19.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 5.44x | 2.05x | 1.13x | 2.49x | 2.28x | 10/10 |
| corpus_materialize | 1.02x | 1.41x | 1.30x | 1.08x | 1.71x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.36x | 3.12x | 2.25x | 3.05x | 1.60x | 10/10 |
| replay_trusted_native | 0.87x | 0.74x | 1.34x | 1.37x | 1.85x | 2.20x | 7/10 |
| replay_checked_native | 0.94x | 2.04x | 1.12x | 8.55x | 8.18x | 9.76x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.46x | 1.87x | 1.63x | 2.52x | 1.92x | 30/30 |
| materialize | 1.04x | 2.12x | 1.57x | 1.75x | 2.60x | 1.89x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.83x | 1.53x | 1.20x | 2.36x | 1.86x | 29/30 |
| perft_full_d3 | 1.01x | 1.43x | 1.51x | 1.54x | 1.97x | 1.62x | 30/30 |
| fen_parse | 0.93x | 1.19x | 1.32x | 1.08x | 1.46x | 1.18x | 29/30 |
| fen_write_native_ep | 1.02x | 4.51x | 10.46x | 2.09x | 30.38x | 2.18x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.32x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 1.03x | 1.92x | N/A | 4.44x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.09x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.16x | 1.36x | 4.12x | 1.36x | 1.72x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.13x | N/A | N/A | 6/6 |
| startpos | 1.12x | 1.11x | 88.22x | 159.23x | 209.50x | 179.44x | 5/5 |
| chess960_materialize | 1.08x | 1.74x | 1.66x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.51x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.09x | 41.55x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.89x | 20.88x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.41x | 2.00x | 1.12x | 2.47x | 2.25x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.30x | 1.09x | 1.77x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.27x | 3.02x | 2.27x | 2.95x | 1.56x | 10/10 |
| replay_trusted_native | 0.97x | 0.86x | 1.52x | 1.59x | 2.18x | 2.51x | 9/10 |
| replay_checked_native | 1.00x | 2.15x | 1.13x | 9.17x | 8.94x | 10.31x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.12x | 3.97x | 1.66x | 1.45x | 2.22x | 1.70x | 30/30 |
| materialize | 1.06x | 2.13x | 1.58x | 1.69x | 2.65x | 1.82x | 30/30 |
| perft_bulk_d4 | 1.27x | 3.59x | 1.42x | 1.12x | 2.20x | 1.73x | 30/30 |
| perft_full_d3 | 1.37x | 0.82x | 0.95x | 0.92x | 1.20x | 0.98x | 13/30 |
| fen_parse | 1.04x | 1.30x | 1.63x | 1.11x | 1.52x | 1.30x | 30/30 |
| fen_write_native_ep | 1.02x | 2.98x | 6.47x | 1.36x | 18.77x | 1.42x | 30/30 |
| snapshot | 2.12x | 1.82x | 1.16x | 4.33x | 1.15x | 1.35x | 30/30 |
| san_parse_all | 2.00x | 1.89x | N/A | 4.31x | N/A | N/A | 12/12 |
| san_render_native | 2.03x | 9.06x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.59x | 1.56x | 0.98x | 2.92x | 0.98x | 1.22x | 24/30 |
| make_unmake_full | 1.27x | N/A | N/A | 0.94x | N/A | N/A | 1/6 |
| startpos | 1.66x | 0.83x | 66.11x | 116.51x | 144.03x | 129.86x | 4/5 |
| chess960_materialize | 1.17x | 1.66x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.34x | 2.11x | 1.24x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.19x | 39.78x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.33x | 21.73x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.19x | 6.59x | 2.47x | 1.36x | 3.03x | 2.75x | 10/10 |
| corpus_materialize | 1.03x | 1.48x | 1.37x | 1.12x | 1.80x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.25x | 1.14x | 1.47x | 1.10x | 1.43x | 0.76x | 8/10 |
| replay_trusted_native | 1.25x | 0.61x | 1.09x | 1.07x | 1.50x | 1.74x | 7/10 |
| replay_checked_native | 1.34x | 1.84x | 0.97x | 7.65x | 7.36x | 8.63x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.

| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.19x | 1.52x | 1.45x | 2.27x | 1.77x | 30/30 |
| materialize | 1.02x | 2.03x | 1.64x | 1.10x | 2.09x | 1.42x | 29/30 |
| perft_bulk_d4 | 0.98x | 3.89x | 1.60x | 1.24x | 2.37x | 2.06x | 30/30 |
| perft_full_d3 | 0.82x | 1.10x | 1.21x | 0.77x | 1.41x | 1.24x | 23/30 |
| fen_parse | 0.98x | 1.27x | 1.53x | 1.10x | 1.84x | 1.33x | 30/30 |
| fen_write_native_ep | 0.98x | 2.85x | 6.03x | 1.19x | 18.45x | 1.38x | 30/30 |
| snapshot | 3.03x | 1.73x | 1.21x | 2.78x | 1.24x | 1.11x | 30/30 |
| san_parse_all | 1.00x | 2.19x | N/A | 3.75x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.95x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.57x | 1.04x | 3.75x | 1.17x | 1.39x | 28/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.09x | 1.18x | 86.25x | 128.74x | 202.68x | 151.43x | 5/5 |
| chess960_materialize | 0.99x | 1.69x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.72x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.58x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.18x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.55x | 1.81x | 1.32x | 2.41x | 2.10x | 10/10 |
| corpus_materialize | 1.00x | 1.90x | 1.76x | 1.22x | 2.83x | 1.54x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.31x | 3.13x | 2.32x | 2.93x | 1.67x | 10/10 |
| replay_trusted_native | 1.00x | 0.76x | 1.26x | 1.45x | 2.61x | 3.32x | 8/10 |
| replay_checked_native | 0.99x | 2.50x | 0.94x | 5.42x | 6.20x | 7.42x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.

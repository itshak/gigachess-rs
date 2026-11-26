| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 3.92x | 1.49x | 1.60x | 1.93x | 1.93x | 30/30 |
| materialize | 1.01x | 1.84x | 1.44x | 1.26x | 2.02x | 1.40x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.42x | 1.39x | 1.27x | 1.80x | 1.83x | 30/30 |
| perft_full_d3 | 1.00x | 1.17x | 1.26x | 1.06x | 1.54x | 1.47x | 29/30 |
| fen_parse | 0.99x | 1.25x | 1.33x | 1.29x | 1.75x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.58x | 9.35x | 1.59x | 29.68x | 2.39x | 30/30 |
| snapshot | 1.07x | 1.22x | 1.02x | 1.61x | 1.03x | 1.02x | 26/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.70x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.12x | 1.31x | 3.61x | 1.13x | 1.08x | 29/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.42x | N/A | N/A | 6/6 |
| startpos | 1.04x | 1.17x | 71.58x | 120.94x | 159.43x | 127.36x | 5/5 |
| chess960_materialize | 1.04x | 1.58x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.65x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.74x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.37x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.82x | 3.92x | 1.52x | 1.21x | 1.78x | 1.96x | 10/10 |
| corpus_materialize | 0.97x | 1.53x | 1.46x | 1.17x | 1.83x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.90x | 2.08x | 2.81x | 1.45x | 10/10 |
| replay_trusted_native | 0.99x | 0.49x | 1.32x | 1.59x | 1.75x | 1.81x | 8/10 |
| replay_checked_native | 0.99x | 2.02x | 0.98x | 6.56x | 5.92x | 7.62x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.

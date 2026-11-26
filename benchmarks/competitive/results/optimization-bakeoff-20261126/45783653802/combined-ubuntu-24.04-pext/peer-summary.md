| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.26x | 1.79x | 1.54x | 2.41x | 1.81x | 30/30 |
| materialize | 1.15x | 2.20x | 1.64x | 1.76x | 2.61x | 1.89x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.08x | 1.62x | 1.27x | 2.53x | 1.98x | 30/30 |
| perft_full_d3 | 1.00x | 1.53x | 1.77x | 1.71x | 2.18x | 1.79x | 30/30 |
| fen_parse | 1.13x | 1.55x | 1.66x | 1.21x | 1.52x | 1.19x | 30/30 |
| fen_write_native_ep | 1.00x | 3.21x | 6.55x | 1.31x | 17.96x | 1.62x | 30/30 |
| snapshot | 1.83x | 1.66x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 0.99x | 1.92x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.14x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 1.71x | 1.10x | 3.30x | 1.04x | 1.39x | 29/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.50x | 1.25x | 98.92x | 178.46x | 208.97x | 172.31x | 5/5 |
| chess960_materialize | 1.08x | 1.72x | 1.61x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.36x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 38.84x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.87x | 19.25x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.76x | 2.49x | 1.36x | 3.12x | 2.85x | 10/10 |
| corpus_materialize | 1.01x | 1.47x | 1.38x | 1.14x | 1.81x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.31x | 2.98x | 2.18x | 2.90x | 1.56x | 10/10 |
| replay_trusted_native | 1.01x | 0.70x | 1.33x | 1.30x | 1.80x | 2.06x | 8/10 |
| replay_checked_native | 1.00x | 2.03x | 1.08x | 8.60x | 8.19x | 9.56x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.

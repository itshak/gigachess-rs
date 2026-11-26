| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.92x | 5.01x | 1.68x | 1.73x | 2.19x | 1.85x | 30/30 |
| materialize | 1.03x | 2.08x | 1.61x | 1.17x | 2.36x | 1.53x | 29/30 |
| perft_bulk_d4 | 0.93x | 4.04x | 1.42x | 1.24x | 1.93x | 1.72x | 30/30 |
| perft_full_d3 | 1.12x | 1.49x | 1.68x | 1.25x | 1.86x | 2.03x | 30/30 |
| fen_parse | 1.17x | 1.25x | 1.36x | 1.45x | 1.74x | 1.56x | 30/30 |
| fen_write_native_ep | 0.96x | 4.00x | 8.73x | 1.69x | 27.27x | 2.13x | 30/30 |
| snapshot | 1.16x | 1.62x | 1.19x | 2.45x | 1.14x | 1.01x | 26/30 |
| san_parse_all | 0.96x | 1.68x | N/A | 3.56x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 11.56x | N/A | 2.84x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.09x | 1.27x | 5.82x | 1.21x | 1.28x | 26/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 0.72x | 0.76x | 60.71x | 96.84x | 134.88x | 109.05x | 4/5 |
| chess960_materialize | 0.92x | 1.54x | 1.74x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.75x | 3.00x | 1.16x | N/A | N/A | N/A | 3/4 |
| polyglot_key | 0.84x | 39.48x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.93x | 1.05x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.84x | 20.35x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 6.42x | 2.35x | 1.39x | 2.50x | 2.93x | 10/10 |
| corpus_materialize | 0.99x | 1.47x | 1.58x | 1.17x | 1.84x | 1.48x | 10/10 |
| corpus_uci_sorted | 0.77x | 1.69x | 2.34x | 1.62x | 2.09x | 1.13x | 10/10 |
| replay_trusted_native | 0.90x | 0.69x | 1.74x | 1.98x | 2.08x | 2.02x | 8/10 |
| replay_checked_native | 0.81x | 2.45x | 1.24x | 7.94x | 7.23x | 10.34x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

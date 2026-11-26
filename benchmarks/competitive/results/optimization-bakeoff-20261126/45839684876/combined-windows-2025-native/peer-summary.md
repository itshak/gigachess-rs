| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.89x | 1.72x | 1.67x | 2.37x | 2.19x | 30/30 |
| materialize | 1.01x | 2.05x | 1.38x | 1.29x | 2.35x | 1.53x | 30/30 |
| perft_bulk_d4 | 0.95x | 3.86x | 1.52x | 1.29x | 2.17x | 1.98x | 30/30 |
| perft_full_d3 | 1.00x | 1.19x | 1.24x | 1.13x | 1.63x | 1.56x | 28/30 |
| fen_parse | 1.14x | 1.32x | 1.39x | 1.35x | 2.19x | 1.44x | 30/30 |
| fen_write_native_ep | 1.03x | 4.00x | 8.09x | 1.55x | 31.22x | 1.95x | 30/30 |
| snapshot | 0.97x | 1.83x | 0.98x | 4.04x | 0.98x | 1.01x | 23/30 |
| san_parse_all | 1.00x | 2.10x | N/A | 5.53x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.86x | N/A | 3.64x | N/A | N/A | 12/12 |
| apply_children_full | 0.90x | 2.04x | 1.24x | 8.27x | 1.31x | 1.47x | 30/30 |
| make_unmake_full | 0.89x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 0.84x | 1.05x | 71.94x | 145.17x | 200.69x | 146.82x | 5/5 |
| chess960_materialize | 1.08x | 1.80x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.97x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 26.80x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 20.09x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 5.62x | 1.89x | 1.39x | 2.57x | 2.53x | 10/10 |
| corpus_materialize | 1.03x | 1.59x | 1.27x | 1.20x | 2.08x | 1.39x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.73x | 2.17x | 1.81x | 2.14x | 1.41x | 10/10 |
| replay_trusted_native | 0.96x | 0.80x | 1.53x | 1.97x | 2.45x | 2.78x | 8/10 |
| replay_checked_native | 0.96x | 2.19x | 0.96x | 7.44x | 7.40x | 8.61x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

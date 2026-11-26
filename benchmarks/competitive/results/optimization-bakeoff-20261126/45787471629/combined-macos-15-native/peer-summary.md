| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.27x | 1.45x | 1.69x | 2.23x | 1.75x | 30/30 |
| materialize | 0.96x | 1.96x | 1.52x | 1.03x | 2.13x | 1.22x | 27/30 |
| perft_bulk_d4 | 1.19x | 4.34x | 1.65x | 1.37x | 2.13x | 2.05x | 30/30 |
| perft_full_d3 | 1.25x | 1.60x | 1.77x | 1.42x | 2.03x | 2.19x | 30/30 |
| fen_parse | 1.10x | 1.26x | 1.36x | 1.39x | 1.78x | 1.48x | 30/30 |
| fen_write_native_ep | 1.01x | 4.40x | 9.26x | 1.78x | 28.39x | 2.20x | 30/30 |
| snapshot | 1.07x | 1.62x | 1.11x | 2.36x | 1.23x | 1.07x | 28/30 |
| san_parse_all | 1.10x | 1.92x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 10.94x | N/A | 2.51x | N/A | N/A | 12/12 |
| apply_children_full | 1.09x | 0.96x | 1.05x | 4.81x | 0.96x | 1.12x | 21/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.14x | N/A | N/A | 5/6 |
| startpos | 0.83x | 1.29x | 63.89x | 142.07x | 173.81x | 136.07x | 5/5 |
| chess960_materialize | 1.00x | 1.54x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 3.18x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.86x | 37.72x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.06x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.89x | 5.82x | 1.93x | 1.18x | 2.16x | 2.44x | 10/10 |
| corpus_materialize | 1.00x | 1.56x | 1.31x | 1.18x | 1.83x | 1.53x | 10/10 |
| corpus_uci_sorted | 0.80x | 1.66x | 2.25x | 1.83x | 2.49x | 1.31x | 10/10 |
| replay_trusted_native | 0.81x | 0.51x | 1.38x | 1.21x | 1.56x | 1.39x | 7/10 |
| replay_checked_native | 0.72x | 2.08x | 0.84x | 6.46x | 6.49x | 6.59x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

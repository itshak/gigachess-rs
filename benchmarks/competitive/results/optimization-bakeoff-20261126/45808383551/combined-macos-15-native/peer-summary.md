| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.31x | 1.39x | 1.50x | 1.92x | 1.64x | 29/30 |
| materialize | 1.11x | 1.99x | 1.52x | 0.96x | 2.27x | 1.33x | 26/30 |
| perft_bulk_d4 | 0.89x | 3.78x | 1.31x | 1.13x | 1.64x | 1.63x | 29/30 |
| perft_full_d3 | 1.15x | 1.56x | 1.80x | 1.39x | 2.02x | 2.05x | 30/30 |
| fen_parse | 1.05x | 1.18x | 1.30x | 1.24x | 1.51x | 1.26x | 29/30 |
| fen_write_native_ep | 1.00x | 4.07x | 8.60x | 1.78x | 27.16x | 2.39x | 30/30 |
| snapshot | 1.09x | 1.68x | 1.05x | 2.44x | 1.17x | 0.97x | 25/30 |
| san_parse_all | 1.11x | 1.93x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 11.43x | N/A | 2.57x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 1.04x | 1.16x | 5.39x | 1.16x | 1.23x | 28/30 |
| make_unmake_full | 1.10x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.41x | 1.17x | 72.00x | 137.22x | 158.54x | 136.20x | 5/5 |
| chess960_materialize | 1.19x | 1.60x | 1.32x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.85x | 1.28x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.16x | 40.92x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.08x | 0.90x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.08x | 21.48x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.66x | 4.32x | 1.56x | 0.94x | 1.83x | 1.90x | 8/10 |
| corpus_materialize | 0.83x | 1.46x | 1.31x | 0.97x | 1.73x | 1.14x | 9/10 |
| corpus_uci_sorted | 0.93x | 1.83x | 2.61x | 1.83x | 2.54x | 1.35x | 10/10 |
| replay_trusted_native | 1.07x | 0.68x | 1.89x | 1.61x | 1.69x | 1.66x | 8/10 |
| replay_checked_native | 0.92x | 2.22x | 1.05x | 8.58x | 6.55x | 7.52x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

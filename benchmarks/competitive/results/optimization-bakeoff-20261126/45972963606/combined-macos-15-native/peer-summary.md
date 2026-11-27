| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.83x | 1.62x | 1.71x | 2.35x | 2.07x | 30/30 |
| materialize | 1.15x | 1.87x | 1.45x | 1.15x | 2.16x | 1.39x | 26/30 |
| perft_bulk_d4 | 0.92x | 4.07x | 1.67x | 1.31x | 1.97x | 1.96x | 29/30 |
| perft_full_d3 | 1.15x | 1.69x | 1.85x | 1.37x | 2.10x | 2.12x | 30/30 |
| fen_parse | 0.99x | 1.27x | 1.30x | 1.46x | 1.73x | 1.48x | 30/30 |
| fen_write_native_ep | 1.00x | 4.20x | 9.21x | 1.86x | 28.18x | 2.31x | 30/30 |
| snapshot | 1.11x | 1.83x | 1.24x | 2.50x | 1.12x | 1.03x | 24/30 |
| san_parse_all | 0.88x | 1.76x | N/A | 3.59x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.61x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.08x | 1.17x | 5.30x | 1.10x | 1.22x | 27/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 0.88x | 1.14x | 73.85x | 165.17x | 203.76x | 169.12x | 5/5 |
| chess960_materialize | 0.96x | 1.59x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.09x | 3.13x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 37.76x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.89x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.10x | 21.38x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 6.19x | 2.08x | 1.07x | 2.10x | 2.37x | 9/10 |
| corpus_materialize | 1.04x | 1.46x | 1.50x | 1.12x | 1.85x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.06x | 2.02x | 2.51x | 1.89x | 2.47x | 1.27x | 10/10 |
| replay_trusted_native | 1.03x | 0.65x | 1.85x | 1.77x | 1.98x | 1.96x | 8/10 |
| replay_checked_native | 0.73x | 2.28x | 1.07x | 7.61x | 6.05x | 7.41x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

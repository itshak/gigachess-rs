| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.69x | 1.96x | 1.70x | 2.59x | 2.01x | 30/30 |
| materialize | 1.10x | 2.28x | 1.69x | 1.78x | 2.67x | 1.89x | 30/30 |
| perft_bulk_d4 | 1.06x | 4.09x | 1.62x | 1.27x | 2.48x | 1.98x | 29/30 |
| perft_full_d3 | 1.20x | 1.63x | 1.94x | 1.83x | 2.36x | 2.02x | 30/30 |
| fen_parse | 1.02x | 1.51x | 1.39x | 1.17x | 1.50x | 1.16x | 30/30 |
| fen_write_native_ep | 1.00x | 4.60x | 10.55x | 2.09x | 28.63x | 2.58x | 30/30 |
| snapshot | 1.00x | 1.66x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.10x | 1.88x | N/A | 4.31x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.37x | N/A | 2.22x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.10x | 1.35x | 4.08x | 1.26x | 1.72x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 99.36x | 178.98x | 216.85x | 177.48x | 5/5 |
| chess960_materialize | 1.01x | 1.77x | 1.68x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.56x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 40.16x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.66x | 1.63x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.37x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.22x | 6.27x | 2.35x | 1.26x | 2.88x | 2.63x | 10/10 |
| corpus_materialize | 1.00x | 1.42x | 1.31x | 1.08x | 1.72x | 1.16x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.27x | 2.94x | 2.20x | 2.85x | 1.56x | 10/10 |
| replay_trusted_native | 1.04x | 0.92x | 1.67x | 1.63x | 2.25x | 2.60x | 9/10 |
| replay_checked_native | 1.00x | 2.19x | 1.18x | 9.18x | 9.11x | 10.29x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

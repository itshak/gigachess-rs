| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.19x | 1.30x | 1.41x | 1.95x | 1.65x | 30/30 |
| materialize | 1.02x | 1.96x | 1.47x | 1.06x | 2.37x | 1.27x | 28/30 |
| perft_bulk_d4 | 1.17x | 4.40x | 1.77x | 1.39x | 2.10x | 2.06x | 30/30 |
| perft_full_d3 | 1.05x | 1.54x | 1.81x | 1.21x | 1.95x | 2.13x | 29/30 |
| fen_parse | 1.06x | 1.18x | 1.22x | 1.25x | 1.61x | 1.51x | 30/30 |
| fen_write_native_ep | 1.00x | 4.12x | 8.69x | 1.78x | 32.50x | 2.20x | 30/30 |
| snapshot | 1.19x | 2.19x | 1.24x | 2.59x | 1.25x | 1.21x | 26/30 |
| san_parse_all | 0.98x | 1.76x | N/A | 4.35x | N/A | N/A | 12/12 |
| san_render_native | 1.08x | 12.42x | N/A | 2.89x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 0.93x | 1.05x | 4.62x | 1.00x | 1.13x | 19/30 |
| make_unmake_full | 1.22x | N/A | N/A | 1.29x | N/A | N/A | 5/6 |
| startpos | 1.51x | 1.55x | 93.85x | 186.24x | 212.56x | 188.00x | 5/5 |
| chess960_materialize | 1.01x | 1.62x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.16x | 3.53x | 1.70x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.90x | 36.93x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.10x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.15x | 22.60x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 5.64x | 2.00x | 1.39x | 2.48x | 3.00x | 10/10 |
| corpus_materialize | 1.12x | 1.68x | 1.56x | 1.11x | 1.90x | 1.30x | 9/10 |
| corpus_uci_sorted | 1.13x | 2.09x | 2.93x | 1.84x | 2.51x | 1.30x | 10/10 |
| replay_trusted_native | 1.23x | 0.60x | 1.57x | 1.67x | 1.75x | 1.67x | 8/10 |
| replay_checked_native | 1.14x | 2.33x | 1.04x | 8.69x | 7.47x | 9.73x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.

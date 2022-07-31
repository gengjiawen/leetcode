#![allow(dead_code)]

extern crate core;

mod _0036_valid_sudoku;
mod _0228_summary_ranges;
mod _0268_missing_number;
mod _0303_range_sum_query_immutable;
mod _0346_moving_average_from_data_stream;
mod _0347_top_k_frequent_elements;
mod _0349_intersection_of_two_arrays;
mod _0350_intersection_of_two_arrays_ii;
mod _0448_find_all_numbers_disappeared_in_an_array;
mod _0463_island_perimeter;
mod _0485_max_consecutive_ones;
mod _0496_next_greater_element_i;
mod _0500_keyboard_row;
mod _0566_reshape_the_matrix;
mod _0575_distribute_candies;
mod _0661_image_smoother;
mod _0682_baseball_game;
mod _0697_degree_of_an_array;
mod _0748_shortest_completing_word;
mod _0760_find_anagram_mappings;
mod _0766_toeplitz_matrix;
mod _0804_unique_morse_code_words;
mod _0806_number_of_lines_to_write_string;
mod _0821_shortest_distance_to_a_character;
mod _0832_flipping_an_image;
mod _0852_peak_index_in_a_mountain_array;
mod _0867_transpose_matrix;
mod _0883_projection_area_of_3d_shapes;
mod _0888_fair_candy_swap;
mod _0892_surface_area_of_3d_shapes;
mod _0896_monotonic_array;
mod _0905_sort_array_by_parity;
mod _0908_smallest_range_i;
mod _0922_sort_array_by_parity_ii;
mod _0929_unique_email_addresses;
mod _0944_delete_columns_to_make_sorted;
mod _0961_n_repeated_element_in_size_2n_array;
mod _0977_squares_of_a_sorted_array;
mod _0999_available_captures_for_rook;
mod _1002_find_common_characters;
mod _1030_matrix_cells_in_distance_order;
mod _1051_height_checker;
mod _1063_number_of_valid_subarrays;
mod _1085_sum_of_digits_in_the_minimum_number;
mod _1089_duplicate_zeros;
mod _1108_defanging_an_ip_address;
mod _1122_relative_sort_array;
mod _1133_largest_unique_number;
mod _1150_check_if_a_number_is_majority_element_in_a_sorted_array;
mod _1160_find_words_that_can_be_formed_by_characters;
mod _1165_single_row_keyboard;
mod _1196_how_many_apples_can_you_put_into_the_basket;
mod _1200_minimum_absolute_difference;
mod _1207_unique_number_of_occurrences;
mod _1213_intersection_of_three_sorted_arrays;
mod _1252_cells_with_odd_values_in_a_matrix;
mod _1266_minimum_time_visiting_all_points;
mod _1268_search_suggestions_system;
mod _1287_element_appearing_more_than_25_in_sorted_array;
mod _1295_find_numbers_with_even_number_of_digits;
mod _1299_replace_elements_with_greatest_element_on_right_side;
mod _1304_find_n_unique_integers_sum_up_to_zero;
mod _1313_decompress_run_length_encoded_list;
mod _1337_the_k_weakest_rows_in_a_matrix;
mod _1351_count_negative_numbers_in_a_sorted_matrix;
mod _1356_sort_integers_by_the_number_of_1_bits;
mod _1365_how_many_numbers_are_smaller_than_the_current_number;
mod _1380_lucky_numbers_in_a_matrix;
mod _1385_find_the_distance_value_between_two_arrays;
mod _1389_create_target_array_in_the_given_order;
mod _1394_find_lucky_integer_in_an_array;
mod _1413_minimum_value_to_get_positive_step_by_step_sum;
mod _1426_counting_elements;
mod _1427_perform_string_shifts;
mod _1431_kids_with_the_greatest_number_of_candies;
mod _1441_build_an_array_with_stack_operations;
mod _1450_number_of_students_doing_homework_at_a_given_time;
mod _1460_make_two_arrays_equal_by_reversing_sub_arrays;
mod _1464_maximum_product_of_two_elements_in_an_array;
mod _1470_shuffle_the_array;
mod _1475_final_prices_with_a_special_discount_in_a_shop;
mod _1480_running_sum_of_1d_array;
mod _1491_average_salary_excluding_the_minimum_and_maximum_salary;
mod _1502_can_make_arithmetic_progression_from_sequence;
mod _1512_number_of_good_pairs;
mod _1528_shuffle_string;
mod _1534_count_good_triplets;
mod _1550_three_consecutive_odds;
mod _1572_matrix_diagonal_sum;
mod _1582_special_positions_in_a_binary_matrix;
mod _1588_sum_of_all_odd_length_subarrays;
mod _1619_mean_of_array_after_removing_some_elements;
mod _1636_sort_array_by_increasing_frequency;
mod _1640_check_array_formation_through_concatenation;
mod _1652_defuse_the_bomb;
mod _1656_design_an_ordered_stream;
mod _1662_check_if_two_string_arrays_are_equivalent;
mod _1672_richest_customer_wealth;
mod _1684_count_the_number_of_consistent_strings;
mod _1700_number_of_students_unable_to_eat_lunch;
mod _1708_largest_subarray_length_k;
mod _1710_maximum_units_on_a_truck;
mod _1720_decode_xored_array;
mod _1725_number_of_rectangles_that_can_form_the_largest_square;
mod _1732_find_the_highest_altitude;
mod _1748_sum_of_unique_elements;
mod _1752_check_if_array_is_sorted_and_rotated;
mod _1773_count_items_matching_a_rule;
mod _1800_maximum_ascending_subarray_sum;
mod _1816_truncate_sentence;
mod _1822_sign_of_the_product_of_an_array;
mod _1827_minimum_operations_to_make_the_array_increasing;
mod _1848_minimum_distance_to_the_target_element;
mod _1854_maximum_population_year;
mod _1863_sum_of_all_subset_xor_totals;
mod _1913_maximum_product_difference_between_two_pairs;
mod _1920_build_array_from_permutation;
mod _1929_concatenation_of_array;
mod _1979_find_greatest_common_divisor_of_array;
mod _1991_find_the_middle_index_in_array;
mod _2006_count_number_of_pairs_with_absolute_difference_k;
mod _2011_final_value_of_variable_after_performing_operations;
mod _2022_convert_1d_array_into_2d_array;
mod _2032_two_out_of_three;
mod _2037_minimum_number_of_moves_to_seat_everyone;
mod _2053_kth_distinct_string_in_an_array;
mod _2057_smallest_index_with_equal_value;
mod _2073_time_needed_to_buy_tickets;
mod _2078_two_furthest_houses_with_different_colors;
mod _2085_count_common_words_with_one_occurrence;
mod _2089_find_target_indices_after_sorting_array;
mod _2108_find_first_palindromic_string_in_the_array;
mod _2114_maximum_number_of_words_found_in_sentences;
mod _2133_check_if_every_row_and_column_contains_all_numbers;
mod _2144_minimum_cost_of_buying_candies_with_discount;
mod _2154_keep_multiplying_found_values_by_two;
mod _2160_minimum_sum_of_four_digit_number_after_splitting_digits;
mod _2164_sort_even_and_odd_indices_independently;
mod _2176_count_equal_and_divisible_pairs_in_an_array;
mod _2185_counting_words_with_a_given_prefix;
mod _2206_divide_array_into_equal_pairs;
mod _2215_find_the_difference_of_two_arrays;
mod _2229_check_if_an_array_is_consecutive;
mod _2235_add_two_integers;
mod _2248_intersection_of_multiple_arrays;
mod _2255_count_prefixes_of_a_given_string;
mod _2293_min_max_game;
mod _2319_check_if_matrix_is_x_matrix;

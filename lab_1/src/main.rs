use std::io;
use std::collections::HashMap;


fn read_int_array() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    numbers
}

fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();

    for i in 0..n {
        let mut swapped = false;

        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        println!("Итерация {}: {:?}", i + 1, arr);

        if !swapped {
            break; // Массив отсортирован. Функция завершается
        }
    }
}


fn find_least_frequent_element(arr: &[i32]) -> i32 {
    let mut frequency_map = HashMap::new();

    // Подсчитываем частоту встреч каждого элемента
    for &num in arr {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    // Находим минимальное количество встреч элемента в массиве
    let least_frequent = frequency_map.values().cloned().min().unwrap_or(0);

    // Ищем элемент(ы), которые встречаются наименьшее количество раз
    let least_frequent_elements: Vec<i32> = frequency_map
        .iter()
        .filter_map(|(&k, &v)| if v == least_frequent { Some(k) } else { None })
        .collect();

    // Возвращаем первый элемент, который встречается реже всего
    *least_frequent_elements.first().unwrap_or(&0)
}

fn count_vowels_consonants(word: &str) -> (usize, usize) {
    let vowels = "aeiouAEIOU";
    let mut vowel_count = 0;
    let mut consonant_count = 0;

    for c in word.chars() {
        if c.is_alphabetic() {
            if vowels.contains(c) {
                vowel_count += 1;
            } else {
                consonant_count += 1;
            }
        }
    }
    (vowel_count, consonant_count)
}

fn main() {
    println!("Введите массив целых чисел через пробел:");
    let numbers = read_int_array();

    let mut numbers_copy = numbers.clone();
    bubble_sort(&mut numbers_copy);

    let least_frequent = find_least_frequent_element(&numbers);

    println!("Самый редко встречающийся элемент: {}", least_frequent);

    // Ввод слов
    let mut input_strings = Vec::new();

    println!("Введите строки (для завершения введите пустую строку):");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка чтения строки");
        let input = input.trim();

        if input.is_empty() {
            break;
        }

        input_strings.push(input.to_string());
    }

    let mut word_frequency = HashMap::new();

    for line in &input_strings {
        let (vowels, consonants) = count_vowels_consonants(line);
        println!(
            "Строка: '{}', Гласные: {}, Согласные: {}",
            line, vowels, consonants
        );

        for word in line.split_whitespace() {
            *word_frequency.entry(word).or_insert(0) += 1;
        }
    }

    if let Some((most_common_word, frequency)) = word_frequency.iter().max_by_key(|&(_, &count)| count) {
        println!("Самое частое слово во всех введенных строках: {} ({} раз)", most_common_word, frequency);
    }

    // Вводим двумерный массив чисел
    let mut input_matrix: Vec<Vec<i32>> = Vec::new();

    println!("Введите двумерный массив целых чисел (чтобы закончить, введите пустую строку):");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error reading the line");
        let input = input.trim();

        if input.is_empty() {
            break;
        }

        let row: Vec<i32> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        input_matrix.push(row);
    }

    // Показываем введённый массив
    println!("Введённый массив:");
    for row in &input_matrix {
        println!("{:?}", row);
    }

    // Сортируем строки по сумме
    input_matrix.sort_by_key(|row| row.iter().sum::<i32>());
    println!("Отсортированные строки:");
    for row in &input_matrix {
        println!("{:?}", row);
    }

    // Набивка строк с малой длиной
    let max_row_len = input_matrix.iter().map(|row| row.len()).max().unwrap_or(0);
    for row in &mut input_matrix {
        while row.len() < max_row_len {
            row.push(0);
        }
    }

}

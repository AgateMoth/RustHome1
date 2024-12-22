use eframe::egui::{ Button, TextEdit};
use crate::{MyApp, Page};

pub fn render(ui: &mut eframe::egui::Ui, app: &mut MyApp) {
    ui.heading("排序器页面");
    ui.label("输入待排序的数字，用逗号分隔:");
    ui.add(TextEdit::singleline(&mut app.sort_input));

    if ui.add(Button::new("冒泡排序")).clicked() {
        let mut numbers: Vec<i32> = app.sort_input
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        bubble_sort(&mut numbers);
        app.sort_result = format!("排序结果: {:?}", numbers);
    }
    if ui.add(Button::new("选择排序")).clicked() {
        let mut numbers: Vec<i32> = app.sort_input
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        selection_sort(&mut numbers);
        app.sort_result = format!("排序结果: {:?}", numbers);
    }
    if ui.add(Button::new("快速排序")).clicked() {
        let mut numbers: Vec<i32> = app.sort_input
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        quick_sort(&mut numbers);
        app.sort_result = format!("排序结果: {:?}", numbers);
    }
    if ui.add(Button::new("希尔排序")).clicked() {
        let mut numbers: Vec<i32> = app.sort_input
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        shell_sort(&mut numbers);
        app.sort_result = format!("排序结果: {:?}", numbers);
    }
    if ui.add(Button::new("归并排序")).clicked() {
        let mut numbers: Vec<i32> = app.sort_input
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        merge_sort(&mut numbers);
        app.sort_result = format!("排序结果: {:?}", numbers);
    }
    // 插入排序
    if ui.add(Button::new("插入排序")).clicked() {
        let mut numbers: Vec<i32> = app.sort_input
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        insertion_sort(&mut numbers);
        app.sort_result = format!("排序结果: {:?}", numbers);
    }
    if ui.add(Button::new("  堆排序  ")).clicked() {
        let mut numbers: Vec<i32> = app.sort_input
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        heap_sort(&mut numbers);
        app.sort_result = format!("排序结果: {:?}", numbers);
    }
    if ui.add(Button::new("计数排序")).clicked() {
        let mut numbers: Vec<i32> = app.sort_input
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        counting_sort(&mut numbers);
        app.sort_result = format!("排序结果: {:?}", numbers);
    }

    ui.label(&app.sort_result);
    if ui.button("返回").clicked() {
        app.current_page = Page::Main;
    }
}

fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn selection_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        // 找到从 i 到 len-1 范围内的最小值的索引
        let mut min_index = i;
        for j in i+1..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        // 将找到的最小值与第 i 个位置的值交换
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}
fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1];
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}



fn shell_sort(arr: &mut [i32]) {
    let n = arr.len();
    let mut gap = n / 2;
    while gap > 0 {
        for i in gap..n {
            let temp = arr[i];
            let mut j = i;
            while j >= gap && arr[j - gap] > temp {
                arr[j] = arr[j - gap];
                j -= gap;
            }
            arr[j] = temp;
        }
        gap /= 2;
    }
}

fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len > 1 {
        let mid = len / 2;
        merge_sort(&mut arr[0..mid]);
        merge_sort(&mut arr[mid..len]);
        merge(arr, mid);
    }
}

fn merge(arr: &mut [i32], mid: usize) {
    let len = arr.len();
    let left = arr[0..mid].to_vec();
    let right = arr[mid..len].to_vec();

    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}


// 插入排序
fn insertion_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

// 堆排序
fn heap_sort(arr: &mut [i32]) {
    let len = arr.len();

    // 构建最大堆
    for i in (0..len / 2).rev() {
        heapify(arr, len, i);
    }

    // 一次一个元素地从堆中取出
    for i in (1..len).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify(arr: &mut [i32], len: usize, i: usize) {
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    let mut largest = i;

    if left < len && arr[left] > arr[largest] {
        largest = left;
    }
    if right < len && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, len, largest);
    }
}

// 计数排序
fn counting_sort(arr: &mut [i32]) {
    let max = match arr.iter().max() {
        Some(&max) => max,
        None => return,
    };

    let mut count = vec![0; (max + 1) as usize];

    // 统计每个元素出现的次数
    for &num in arr.iter() {
        count[num as usize] += 1;
    }

    // 填充原数组
    let mut idx = 0;
    for (num, &cnt) in count.iter().enumerate() {
        for _ in 0..cnt {
            arr[idx] = num as i32;
            idx += 1;
        }
    }
}

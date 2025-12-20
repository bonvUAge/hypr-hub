#!/bin/bash

# Запрашиваем путь к файлу у пользователя
echo "Type a path to an image (or drag and drop the file into the terminal):"
read -r IMG_PATH

# Убираем лишние кавычки, если пользователь перетащил файл
IMG_PATH="${IMG_PATH//\'/}"
IMG_PATH="${IMG_PATH//\"/}"

# Проверяем, существует ли файл
if [[ -f "$IMG_PATH" ]]; then
    # Выполняем современную команду reload для всех мониторов (через запятую)
    hyprctl hyprpaper reload ,"$IMG_PATH"
    echo "Wallpapers updated: $IMG_PATH"
else
    echo "Error: File at path '$IMG_PATH' not found!"
    exit 1
fi

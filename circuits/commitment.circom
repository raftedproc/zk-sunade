pragma circom 2.0.0;

template Commitment() {
    signal input secret;    // Секретное значение
    signal input note;      // Публичная информация
    signal output commitment; // Коммитмент

    commitment <== secret * note; // Пример упрощенной операции
}

component main = Commitment();

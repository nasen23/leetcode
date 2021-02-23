#include "x.h"

#include <mutex>
#include <condition_variable>

class Foo {
private:
  std::mutex mu;
  std::condition_variable cv;
  int signal = 0;

public:
  Foo() {}

  void first(function<void()> printFirst) {
    // printFirst() outputs "first". Do not change or remove this line.
    printFirst();
    signal = 1;
    cv.notify_all();
  }

  void second(function<void()> printSecond) {
    std::unique_lock<std::mutex> lock(mu);
    cv.wait(lock, [&] { return signal == 1; });

    // printSecond() outputs "second". Do not change or remove this line.
    printSecond();
    signal = 2;
    cv.notify_one();
  }

  void third(function<void()> printThird) {
    std::unique_lock<std::mutex> lock(mu);
    cv.wait(lock, [&] { return signal == 2; });

    // printThird() outputs "third". Do not change or remove this line.
    printThird();
  }
};

#include <memory>

template <typename T, typename... Args>
std::unique_ptr<T> construct_unique(Args... args) {
  return std::unique_ptr<T>(new T(args...));
}
